;; Pi-hosted Ralph loop for draining native Cairn changes across sibling repos.
;;
;; Plan mode is the default. --run launches one fresh `pi -p` process per
;; bounded round. Pi may drain at most one dependency-ready change in the
;; selected repository; this supervisor then independently compares Cairn
;; lifecycle state. This is an operator-invoked temporary host helper, not the
;; embedded Animus Steel runtime or Cairn's trusted orchestration boundary.

(require "steel/result")
(require-builtin steel/process)

(define default-workspace-root "..")
(define default-round-limit 1)
(define unlimited-round-limit 0)
(define initial-round-count 0)
(define first-display-round 1)
(define expected-single-change-count 1)
(define sample-open-task-count 2)
(define sample-limit-value 3)
(define failing-status-probe 7)
(define default-pi-timeout-seconds 3600)
(define pi-timeout-kill-grace-seconds 30)
(define pi-timeout-exit-status 124)
(define default-agent-program "pi")
(define timeout-program "timeout")
(define timeout-signal-argument "--signal=TERM")
(define pi-print-argument "-p")
(define path-separator "/")
(define changes-relative-path "cairn/changes")
(define archive-relative-path "cairn/archive")
(define tasks-file-name "tasks.md")
(define unchecked-task-prefix "- [ ]")
(define process-cd-script "cd \"$1\" && shift && exec \"$@\" < /dev/null")
(define process-shell-name "sh")
(define canonical-cairn-repo-name "cairn")
(define canonical-cairn-app-suffix "#cairn")

(define progress-archived "archived")
(define progress-task "task-progress")
(define progress-none "no-progress")
(define progress-scope-breach "scope-breach")

(define disallowed-pi-exact-args
  (list
   "-p"
   "--print"
   "-c"
   "--continue"
   "-r"
   "--resume"
   "--session"
   "--fork"
   "--name"
   "--mode"
   "--"))

(define disallowed-pi-prefix-args
  (list
   "--session="
   "--fork="
   "--name="
   "--mode="))

(struct Config
  (workspace-root round-limit agent-program pi-args run? repos self-test? help?))

(struct Repo (name root))
(struct ChangeSnapshot (name unchecked-task-count))
(struct RepoSnapshot (repo changes archives))
(struct Progress (kind change archive detail))

(define (say value)
  (display value)
  (newline))

(define (string-prefix? prefix text)
  (let ([prefix-length (string-length prefix)]
        [text-length (string-length text)])
    (and (<= prefix-length text-length)
         (string=? prefix (substring text 0 prefix-length)))))

(define (string-suffix? suffix text)
  (let ([suffix-length (string-length suffix)]
        [text-length (string-length text)])
    (and (<= suffix-length text-length)
         (string=? suffix
                   (substring text (- text-length suffix-length) text-length)))))

(define (ends-with-slash? text)
  (let ([text-length (string-length text)])
    (and (> text-length 0)
         (char=? (string-ref text (- text-length 1)) #\/))))

(define (path-join left right)
  (if (ends-with-slash? left)
      (string-append left right)
      (string-append left path-separator right)))

(define (last-slash-index text)
  (let loop ([index (- (string-length text) 1)])
    (cond
      [(< index 0) #false]
      [(char=? (string-ref text index) #\/) index]
      [else (loop (- index 1))])))

(define (basename path)
  (let ([slash-index (last-slash-index path)])
    (if slash-index
        (substring path (+ slash-index 1) (string-length path))
        path)))

(define (join-lines lines)
  (if (null? lines)
      ""
      (string-append (car lines) "\n" (join-lines (cdr lines)))))

(define (parse-limit-value value)
  (let ([parsed (string->number value)])
    (if (and parsed (integer? parsed) (>= parsed unlimited-round-limit))
        parsed
        #false)))

(define (usage-lines)
  (list
   "Run with Pi's Steel tool:"
   "  file: scripts/ralph-drain-stack.scm"
   "  args: [ ...options... ]"
   ""
   "Options:"
   "  --workspace-root PATH  Parent containing sibling repositories (default: ..)"
   "  --limit N              Pi rounds to run; 0 means until complete/blocked (default: 1)"
   "  --repo NAME            Select one discovered native Cairn repo; repeat to set order"
   "  --agent PROGRAM        Pi executable used by --run (default: pi)"
   "  --pi-arg ARG           Extra Pi argument; repeat to preserve argument boundaries"
   "  --run                  Launch fresh Pi print-mode sessions"
   "  --self-test            Run positive and negative pure-core tests"
   "  --help                 Show this help"
   ""
   "Examples:"
   "  args: [\"--workspace-root\", \"..\"]"
   "  args: [\"--workspace-root\", \"..\", \"--repo\", \"cairn\", \"--run\", \"--limit\", \"1\"]"
   "  args: [\"--self-test\"]"))

(define (parse-args raw-args)
  (let loop ([args raw-args]
             [workspace-root default-workspace-root]
             [round-limit default-round-limit]
             [agent-program default-agent-program]
             [extra-pi-args '()]
             [run? #false]
             [repos '()]
             [self-test? #false]
             [help? #false])
    (cond
      [(null? args)
       (Config workspace-root
               round-limit
               agent-program
               (reverse extra-pi-args)
               run?
               (reverse repos)
               self-test?
               help?)]
      [(string=? (car args) "--workspace-root")
       (if (null? (cdr args))
           (error "--workspace-root requires a path")
           (loop (cddr args)
                 (cadr args)
                 round-limit
                 agent-program
                 extra-pi-args
                 run?
                 repos
                 self-test?
                 help?))]
      [(string-prefix? "--workspace-root=" (car args))
       (loop (cdr args)
             (substring (car args)
                        (string-length "--workspace-root=")
                        (string-length (car args)))
             round-limit
             agent-program
             extra-pi-args
             run?
             repos
             self-test?
             help?)]
      [(string=? (car args) "--limit")
       (if (null? (cdr args))
           (error "--limit requires a non-negative integer")
           (let ([parsed-limit (parse-limit-value (cadr args))])
             (if parsed-limit
                 (loop (cddr args)
                       workspace-root
                       parsed-limit
                       agent-program
                       extra-pi-args
                       run?
                       repos
                       self-test?
                       help?)
                 (error "--limit requires a non-negative integer"))))]
      [(string-prefix? "--limit=" (car args))
       (let ([parsed-limit
              (parse-limit-value
               (substring (car args)
                          (string-length "--limit=")
                          (string-length (car args))))])
         (if parsed-limit
             (loop (cdr args)
                   workspace-root
                   parsed-limit
                   agent-program
                   extra-pi-args
                   run?
                   repos
                   self-test?
                   help?)
             (error "--limit requires a non-negative integer")))]
      [(string=? (car args) "--repo")
       (if (null? (cdr args))
           (error "--repo requires a repository name")
           (loop (cddr args)
                 workspace-root
                 round-limit
                 agent-program
                 extra-pi-args
                 run?
                 (cons (cadr args) repos)
                 self-test?
                 help?))]
      [(string-prefix? "--repo=" (car args))
       (loop (cdr args)
             workspace-root
             round-limit
             agent-program
             extra-pi-args
             run?
             (cons (substring (car args)
                              (string-length "--repo=")
                              (string-length (car args)))
                   repos)
             self-test?
             help?)]
      [(string=? (car args) "--agent")
       (if (null? (cdr args))
           (error "--agent requires a program")
           (loop (cddr args)
                 workspace-root
                 round-limit
                 (cadr args)
                 extra-pi-args
                 run?
                 repos
                 self-test?
                 help?))]
      [(string-prefix? "--agent=" (car args))
       (loop (cdr args)
             workspace-root
             round-limit
             (substring (car args)
                        (string-length "--agent=")
                        (string-length (car args)))
             extra-pi-args
             run?
             repos
             self-test?
             help?)]
      [(string=? (car args) "--pi-arg")
       (if (null? (cdr args))
           (error "--pi-arg requires one argument value")
           (loop (cddr args)
                 workspace-root
                 round-limit
                 agent-program
                 (cons (cadr args) extra-pi-args)
                 run?
                 repos
                 self-test?
                 help?))]
      [(string-prefix? "--pi-arg=" (car args))
       (loop (cdr args)
             workspace-root
             round-limit
             agent-program
             (cons (substring (car args)
                              (string-length "--pi-arg=")
                              (string-length (car args)))
                   extra-pi-args)
             run?
             repos
             self-test?
             help?)]
      [(string=? (car args) "--run")
       (loop (cdr args)
             workspace-root
             round-limit
             agent-program
             extra-pi-args
             #true
             repos
             self-test?
             help?)]
      [(string=? (car args) "--self-test")
       (loop (cdr args)
             workspace-root
             round-limit
             agent-program
             extra-pi-args
             run?
             repos
             #true
             help?)]
      [(string=? (car args) "--help")
       (loop (cdr args)
             workspace-root
             round-limit
             agent-program
             extra-pi-args
             run?
             repos
             self-test?
             #true)]
      [else (error (string-append "unknown argument: " (car args)))])))

(define (collect-directory-paths root)
  (if (not (path-exists? root))
      '()
      (let ([iter (read-dir-iter root)])
        (let loop ([paths '()])
          (let ([entry (read-dir-iter-next! iter)])
            (cond
              [(not entry) (sort paths string<?)]
              [(read-dir-entry-is-dir? entry)
               (loop (cons (read-dir-entry-path entry) paths))]
              [else (loop paths)]))))))

(define (native-cairn-repo-path? path)
  (path-exists? (path-join path changes-relative-path)))

(define (repo-path->repo path)
  (Repo (basename path) path))

(define (discover-repos workspace-root)
  (map repo-path->repo
       (filter native-cairn-repo-path?
               (collect-directory-paths workspace-root))))

(define (find-repo name repos)
  (cond
    [(null? repos) #false]
    [(string=? name (Repo-name (car repos))) (car repos)]
    [else (find-repo name (cdr repos))]))

(define (all-selected-known? selected-names discovered-repos)
  (or (null? selected-names)
      (and (find-repo (car selected-names) discovered-repos)
           (all-selected-known? (cdr selected-names) discovered-repos))))

(define (duplicates? values)
  (cond
    [(null? values) #false]
    [(member (car values) (cdr values)) #true]
    [else (duplicates? (cdr values))]))

(define (select-repos discovered-repos selected-names)
  (cond
    [(null? selected-names) discovered-repos]
    [(duplicates? selected-names)
     (error "--repo values must not contain duplicates")]
    [(not (all-selected-known? selected-names discovered-repos))
     (error "--repo named an unknown or non-Cairn repository")]
    [else (map (lambda (name) (find-repo name discovered-repos)) selected-names)]))

(define (count-unchecked-task-lines lines)
  (cond
    [(null? lines) 0]
    [(string-prefix? unchecked-task-prefix (car lines))
     (+ 1 (count-unchecked-task-lines (cdr lines)))]
    [else (count-unchecked-task-lines (cdr lines))]))

(define (file-lines path)
  (if (not (path-exists? path))
      '()
      (let ([port (open-input-file path)])
        (let loop ([lines '()])
          (let ([line (read-line port)])
            (if (eof-object? line)
                (begin
                  (close-input-port port)
                  (reverse lines))
                (loop (cons line lines))))))))

(define (change-path->snapshot path)
  (ChangeSnapshot
   (basename path)
   (count-unchecked-task-lines
    (file-lines (path-join path tasks-file-name)))))

(define (repo-change-snapshots repo)
  (map change-path->snapshot
       (collect-directory-paths
        (path-join (Repo-root repo) changes-relative-path))))

(define (repo-archive-names repo)
  (map basename
       (collect-directory-paths
        (path-join (Repo-root repo) archive-relative-path))))

(define (snapshot-repo repo)
  (RepoSnapshot repo
                (repo-change-snapshots repo)
                (repo-archive-names repo)))

(define (change-names changes)
  (map ChangeSnapshot-name changes))

(define (list-difference left right)
  (filter (lambda (value) (not (member value right))) left))

(define (find-change-snapshot name changes)
  (cond
    [(null? changes) #false]
    [(string=? name (ChangeSnapshot-name (car changes))) (car changes)]
    [else (find-change-snapshot name (cdr changes))]))

(define (changed-common-task-names before-changes after-changes)
  (let loop ([remaining before-changes] [changed '()])
    (if (null? remaining)
        (reverse changed)
        (let* ([before (car remaining)]
               [after (find-change-snapshot (ChangeSnapshot-name before) after-changes)])
          (if (and after
                   (not (= (ChangeSnapshot-unchecked-task-count before)
                           (ChangeSnapshot-unchecked-task-count after))))
              (loop (cdr remaining) (cons (ChangeSnapshot-name before) changed))
              (loop (cdr remaining) changed))))))

(define (task-count-decreased? name before-changes after-changes)
  (let ([before (find-change-snapshot name before-changes)]
        [after (find-change-snapshot name after-changes)])
    (and before
         after
         (< (ChangeSnapshot-unchecked-task-count after)
            (ChangeSnapshot-unchecked-task-count before)))))

(define (matching-archive-name? archive-name change-name)
  (string-suffix? (string-append "-" change-name) archive-name))

(define (classify-progress before after)
  (let* ([before-changes (RepoSnapshot-changes before)]
         [after-changes (RepoSnapshot-changes after)]
         [before-names (change-names before-changes)]
         [after-names (change-names after-changes)]
         [removed-changes (list-difference before-names after-names)]
         [added-changes (list-difference after-names before-names)]
         [changed-task-names
          (changed-common-task-names before-changes after-changes)]
         [new-archives
          (list-difference (RepoSnapshot-archives after)
                           (RepoSnapshot-archives before))]
         [removed-archives
          (list-difference (RepoSnapshot-archives before)
                           (RepoSnapshot-archives after))])
    (cond
      [(not (null? added-changes))
       (Progress progress-scope-breach #false #false
                 "Pi added an active Cairn change")]
      [(not (null? removed-archives))
       (Progress progress-scope-breach #false #false
                 "Pi removed historical Cairn archive state")]
      [(> (length removed-changes) expected-single-change-count)
       (Progress progress-scope-breach #false #false
                 "Pi removed more than one active Cairn change")]
      [(= (length removed-changes) expected-single-change-count)
       (let ([removed-change (car removed-changes)])
         (cond
           [(not (null? changed-task-names))
            (Progress progress-scope-breach removed-change #false
                      "Pi archived one change and mutated task state in another")]
           [(not (= (length new-archives) expected-single-change-count))
            (Progress progress-scope-breach removed-change #false
                      "removed change did not create exactly one new archive")]
           [(not (matching-archive-name? (car new-archives) removed-change))
            (Progress progress-scope-breach removed-change (car new-archives)
                      "new archive does not match the removed change")]
           [else
            (Progress progress-archived removed-change (car new-archives)
                      "one active change archived")]))]
      [(not (null? new-archives))
       (Progress progress-scope-breach #false (car new-archives)
                 "Pi created archive state without removing one active change")]
      [(> (length changed-task-names) expected-single-change-count)
       (Progress progress-scope-breach #false #false
                 "Pi changed task state in more than one active change")]
      [(= (length changed-task-names) expected-single-change-count)
       (let ([changed-name (car changed-task-names)])
         (if (task-count-decreased? changed-name before-changes after-changes)
             (Progress progress-task changed-name #false
                       "unchecked task count decreased in one active change")
             (Progress progress-scope-breach changed-name #false
                       "unchecked task count increased")))]
      [else
       (Progress progress-none #false #false
                 "Pi exited without a Cairn lifecycle progress marker")]))
  )

(define (repo-active? snapshot)
  (not (null? (RepoSnapshot-changes snapshot))))

(define (repo-blocked? repo blocked-repo-names)
  (member (Repo-name repo) blocked-repo-names))

(define (first-dispatchable-snapshot snapshots blocked-repo-names)
  (cond
    [(null? snapshots) #false]
    [(and (repo-active? (car snapshots))
          (not (repo-blocked? (RepoSnapshot-repo (car snapshots))
                              blocked-repo-names)))
     (car snapshots)]
    [else
     (first-dispatchable-snapshot (cdr snapshots) blocked-repo-names)]))

(define (total-active-change-count snapshots)
  (apply + (map (lambda (snapshot) (length (RepoSnapshot-changes snapshot))) snapshots)))

(define (bounded-rounds-exhausted? round-limit rounds-used)
  (and (> round-limit unlimited-round-limit)
       (>= rounds-used round-limit)))

(define (pi-arg-disallowed? value)
  (or (member value disallowed-pi-exact-args)
      (let loop ([prefixes disallowed-pi-prefix-args])
        (cond
          [(null? prefixes) #false]
          [(string-prefix? (car prefixes) value) #true]
          [else (loop (cdr prefixes))]))))

(define (validate-pi-args extra-pi-args)
  (for-each
   (lambda (value)
     (when (pi-arg-disallowed? value)
       (error (string-append
               "--pi-arg cannot override print mode, session freshness, name, or mode: "
               value))))
   extra-pi-args))

(define (ralph-prompt-lines repo)
  (list
   (string-append
    "Goal: drain at most one dependency-ready active Cairn change in repository `"
    (Repo-name repo)
    "` rooted at `"
    (Repo-root repo)
    "`.")
   ""
   "Success: either (a) exactly one change is implemented, its verified tasks are checked, accepted specs are synced, it is archived, Cairn validation passes, and a why-focused local commit is created; or (b) no unsafe mutation is made and one concrete blocker is reported."
   ""
   "Constraints:"
   "- Load and follow the `cairn` and `cairn-drain` skills, local `AGENTS.md`, and `README.md` before editing."
   "- Treat this repository as an independent VCS and lifecycle root; do not edit sibling repositories."
   "- Use the repository-pinned Cairn command when available and inspect `change next` or the read-only drain plan before selecting work."
   "- Work on at most one change. Do not create a new change package or begin a second active change."
   "- Preserve pre-existing user changes. If safe attribution is unclear, stop with a blocker instead of modifying the worktree."
   "- Run the smallest relevant baseline before core edits, preserve functional-core/imperative-shell boundaries, apply Tiger Style, and add positive and negative tests."
   "- Dry-run sync and archive before explicit execution; verify accepted requirement text landed; validate after archive."
   "- Never push, force-push, or open/manage a pull request."
   "- Do not claim cross-repository dependency satisfaction or stack readiness."
   ""
   "Report: selected change, archive path if completed, checks run, commit if created, blocker if any, preserved user changes, and remaining active change count."))

(define (ralph-prompt repo)
  (join-lines (ralph-prompt-lines repo)))

(define (external-command-status root program args)
  (unwrap-ok
   (process-wait
    (unwrap-ok
     (spawn-process
      (command process-shell-name
               (append
                (list "-c" process-cd-script process-shell-name root program)
                args)))))))

(define (seconds-argument seconds)
  (string-append (number->string seconds) "s"))

(define (kill-after-argument seconds)
  (string-append "--kill-after=" (seconds-argument seconds)))

(define (run-pi-round! config display-round repo)
  (let* ([session-name
          (string-append "Stack Cairn drain: " (Repo-name repo)
                         " round " (number->string display-round))]
         [pi-args
          (append
           (list pi-print-argument)
           (Config-pi-args config)
           (list "--name" session-name (ralph-prompt repo)))]
         [timeout-args
          (append
           (list timeout-signal-argument
                 (kill-after-argument pi-timeout-kill-grace-seconds)
                 (seconds-argument default-pi-timeout-seconds)
                 (Config-agent-program config))
           pi-args)])
    (say (string-append "==> Pi round " (number->string display-round)
                        " repository=" (Repo-name repo)
                        " timeout_seconds="
                        (number->string default-pi-timeout-seconds)))
    (external-command-status
     (Repo-root repo)
     timeout-program
     timeout-args)))

(define (canonical-cairn-app workspace-root)
  (string-append
   "path:"
   (path-join workspace-root canonical-cairn-repo-name)
   canonical-cairn-app-suffix))

(define (validate-repo-with-cairn! workspace-root repo)
  (say (string-append "==> Cairn validate repository=" (Repo-name repo)))
  (external-command-status
   (Repo-root repo)
   "nix"
   (list "run"
         (canonical-cairn-app workspace-root)
         "--"
         "validate"
         "--root"
         (Repo-root repo))))

(define (repo-plan-line snapshot)
  (string-append
   "- "
   (Repo-name (RepoSnapshot-repo snapshot))
   ": "
   (number->string (length (RepoSnapshot-changes snapshot)))
   " active change(s)"))

(define (plan-lines config workspace-root repos snapshots)
  (append
   (list
    "# Pi-hosted Stack Cairn Ralph Loop"
    ""
    (string-append "Workspace root: `" workspace-root "`")
    (string-append "Mode: `" (if (Config-run? config) "run" "plan-only") "`")
    (string-append "Round limit: "
                   (if (= (Config-round-limit config) unlimited-round-limit)
                       "until complete or blocked"
                       (number->string (Config-round-limit config))))
    (string-append "Selected repositories: " (number->string (length repos)))
    (string-append "Active changes: " (number->string (total-active-change-count snapshots)))
    ""
    "Repositories:")
   (map repo-plan-line snapshots)
   (list
    ""
    "Plan mode performs no Pi, Cairn, filesystem-mutation, or VCS effects. Pass `--run` explicitly to launch fresh Pi print-mode sessions."
    ""
    "Ordering is deterministic but advisory. It does not prove cross-repository dependency satisfaction.")))

(define (print-lines lines)
  (for-each say lines))

(struct RoundOutcome (block-repo? archived-increment))

(define (scope-breach-error repo progress)
  (error
   (string-append
    "scope breach in repository "
    (Repo-name repo)
    ": "
    (Progress-detail progress))))

(define (validate-archived-progress! workspace-root repo progress pi-status)
  (say (string-append "PROGRESS repository=" (Repo-name repo)
                      " change=" (Progress-change progress)
                      " archive=" (Progress-archive progress)
                      " pi_status=" (number->string pi-status)))
  (let ([validation-status (validate-repo-with-cairn! workspace-root repo)])
    (when (not (= validation-status 0))
      (error
       (string-append
        "post-archive Cairn validation failed for "
        (Repo-name repo)
        " with exit "
        (number->string validation-status)))))
  (RoundOutcome #false 1))

(define (handle-round-progress! workspace-root repo progress pi-status)
  (cond
    [(string=? (Progress-kind progress) progress-scope-breach)
     (scope-breach-error repo progress)]
    [(string=? (Progress-kind progress) progress-archived)
     (validate-archived-progress! workspace-root repo progress pi-status)]
    [(string=? (Progress-kind progress) progress-task)
     (say (string-append "PROGRESS repository=" (Repo-name repo)
                         " change=" (Progress-change progress)
                         " kind=task-progress"
                         " pi_status=" (number->string pi-status)))
     (RoundOutcome #false 0)]
    [(not (= pi-status 0))
     (say (string-append "BLOCK repository=" (Repo-name repo)
                         " reason="
                         (if (= pi-status pi-timeout-exit-status)
                             "pi-timeout"
                             "pi-exit")
                         " status="
                         (number->string pi-status)))
     (RoundOutcome #true 0)]
    [else
     (say (string-append "BLOCK repository=" (Repo-name repo)
                         " reason=no-lifecycle-progress"))
     (RoundOutcome #true 0)]))

(define (execute-round! config workspace-root display-round before)
  (let* ([repo (RepoSnapshot-repo before)]
         [pi-status (run-pi-round! config display-round repo)]
         [after (snapshot-repo repo)]
         [progress (classify-progress before after)])
    (handle-round-progress! workspace-root repo progress pi-status)))

(define (next-blocked-repos repo blocked-repo-names outcome)
  (if (RoundOutcome-block-repo? outcome)
      (cons (Repo-name repo) blocked-repo-names)
      blocked-repo-names))

(define (run-stack! config workspace-root repos)
  (validate-pi-args (Config-pi-args config))
  (when (not (path-exists? (path-join workspace-root canonical-cairn-repo-name)))
    (error "canonical sibling Cairn repository is missing"))
  (say "Pi-hosted stack Cairn drain: starting serial rounds")
  (let loop ([rounds-used initial-round-count]
             [blocked-repo-names '()]
             [archived-count 0])
    (let* ([snapshots (map snapshot-repo repos)]
           [active-count (total-active-change-count snapshots)]
           [candidate
            (first-dispatchable-snapshot snapshots blocked-repo-names)])
      (cond
        [(= active-count 0)
         (say (string-append "COMPLETE archived=" (number->string archived-count)
                             " remaining=0"))]
        [(bounded-rounds-exhausted? (Config-round-limit config) rounds-used)
         (say (string-append "BUDGET_EXHAUSTED rounds=" (number->string rounds-used)
                             " archived=" (number->string archived-count)
                             " remaining=" (number->string active-count)))]
        [(not candidate)
         (say (string-append "BLOCKED archived=" (number->string archived-count)
                             " remaining=" (number->string active-count)
                             " blocked_repositories="
                             (number->string (length blocked-repo-names))))]
        [else
         (let* ([repo (RepoSnapshot-repo candidate)]
                [display-round (+ rounds-used first-display-round)]
                [outcome
                 (execute-round! config workspace-root display-round candidate)])
           (loop (+ rounds-used 1)
                 (next-blocked-repos repo blocked-repo-names outcome)
                 (+ archived-count
                    (RoundOutcome-archived-increment outcome))))]))))

(define (check label condition)
  (if condition
      (say (string-append "ok - " label))
      (error (string-append "self-test failed: " label))))

(define (fixture-repo name)
  (Repo name (string-append "/tmp/" name)))

(define (fixture-change name unchecked-task-count)
  (ChangeSnapshot name unchecked-task-count))

(define (fixture-snapshot repo changes archives)
  (RepoSnapshot repo changes archives))

(define (run-self-test)
  (let* ([alpha-repo (fixture-repo "alpha")]
         [beta-repo (fixture-repo "beta")]
         [discovered (list alpha-repo beta-repo)]
         [before
          (fixture-snapshot
           alpha-repo
           (list (fixture-change "first" sample-open-task-count)
                 (fixture-change "second" 1))
           (list "2026-01-01-old"))]
         [task-after
          (fixture-snapshot
           alpha-repo
           (list (fixture-change "first" 1)
                 (fixture-change "second" 1))
           (list "2026-01-01-old"))]
         [archive-after
          (fixture-snapshot
           alpha-repo
           (list (fixture-change "second" 1))
           (list "2026-01-01-old" "2026-01-02-first"))]
         [added-after
          (fixture-snapshot
           alpha-repo
           (list (fixture-change "first" sample-open-task-count)
                 (fixture-change "second" 1)
                 (fixture-change "third" 1))
           (list "2026-01-01-old"))]
         [multiple-removed-after
          (fixture-snapshot
           alpha-repo
           '()
           (list "2026-01-01-old" "2026-01-02-first" "2026-01-02-second"))]
         [multiple-task-after
          (fixture-snapshot
           alpha-repo
           (list (fixture-change "first" 1)
                 (fixture-change "second" 0))
           (list "2026-01-01-old"))]
         [missing-archive-after
          (fixture-snapshot
           alpha-repo
           (list (fixture-change "second" 1))
           (list "2026-01-01-old"))]
         [removed-archive-after
          (fixture-snapshot
           alpha-repo
           (list (fixture-change "first" sample-open-task-count)
                 (fixture-change "second" 1))
           '())])
    (check "prefix positive" (string-prefix? "--repo=" "--repo=alpha"))
    (check "prefix negative" (not (string-prefix? "--repo=" "--root=alpha")))
    (check "suffix positive" (string-suffix? "-first" "2026-01-02-first"))
    (check "suffix negative" (not (string-suffix? "-first" "2026-01-02-second")))
    (check "path join" (string=? (path-join "/tmp" "alpha") "/tmp/alpha"))
    (check "limit parse positive"
           (= (parse-limit-value (number->string sample-limit-value))
              sample-limit-value))
    (check "limit parse negative" (not (parse-limit-value "1.5")))
    (check "selected repo order"
           (equal? (map Repo-name (select-repos discovered (list "beta" "alpha")))
                   (list "beta" "alpha")))
    (check "unknown repo rejected by predicate"
           (not (all-selected-known? (list "gamma") discovered)))
    (check "duplicate repo detected" (duplicates? (list "alpha" "alpha")))
    (check "unchecked task count positive"
           (= (count-unchecked-task-lines
               (list "- [ ] first" "- [x] done" "text" "- [ ] second"))
              sample-open-task-count))
    (check "unchecked task count negative"
           (= (count-unchecked-task-lines (list "- [x] done" "text")) 0))
    (check "fresh Pi arg positive" (not (pi-arg-disallowed? "--model")))
    (check "fresh Pi arg exact negative" (pi-arg-disallowed? "--continue"))
    (check "fresh Pi arg prefix negative" (pi-arg-disallowed? "--session=old"))
    (check "fresh Pi argument terminator negative" (pi-arg-disallowed? "--"))
    (check "task progress positive"
           (string=? (Progress-kind (classify-progress before task-after))
                     progress-task))
    (check "archive progress positive"
           (string=? (Progress-kind (classify-progress before archive-after))
                     progress-archived))
    (check "no progress positive"
           (string=? (Progress-kind (classify-progress before before))
                     progress-none))
    (check "added change negative"
           (string=? (Progress-kind (classify-progress before added-after))
                     progress-scope-breach))
    (check "multiple removals negative"
           (string=? (Progress-kind (classify-progress before multiple-removed-after))
                     progress-scope-breach))
    (check "multiple task changes negative"
           (string=? (Progress-kind (classify-progress before multiple-task-after))
                     progress-scope-breach))
    (check "missing archive negative"
           (string=? (Progress-kind (classify-progress before missing-archive-after))
                     progress-scope-breach))
    (check "removed archive negative"
           (string=? (Progress-kind (classify-progress before removed-archive-after))
                     progress-scope-breach))
    (check "dispatch skips blocked repository"
           (string=?
            (Repo-name
             (RepoSnapshot-repo
              (first-dispatchable-snapshot
               (list before (fixture-snapshot beta-repo (list (fixture-change "work" 1)) '()))
               (list "alpha"))))
            "beta"))
    (check "external command success"
           (= (external-command-status "." "true" '()) 0))
    (check "external command receives EOF"
           (= (external-command-status
               "."
               "sh"
               (list "-c" "if read value; then exit 1; else exit 0; fi"))
              0))
    (check "external command failure"
           (= (external-command-status
               "."
               "sh"
               (list "-c"
                     (string-append "exit "
                                    (number->string failing-status-probe))))
              failing-status-probe))
    (check "Pi timeout argument"
           (string=? (seconds-argument default-pi-timeout-seconds)
                     "3600s"))
    (say "self-test complete")))

(define (script-args)
  pi-args)

(define (main)
  (let ([config (parse-args (script-args))])
    (cond
      [(Config-help? config) (print-lines (usage-lines))]
      [(Config-self-test? config) (run-self-test)]
      [else
       (let* ([workspace-root (canonicalize-path (Config-workspace-root config))]
              [discovered-repos (discover-repos workspace-root)]
              [selected-repos
               (select-repos discovered-repos (Config-repos config))]
              [snapshots (map snapshot-repo selected-repos)])
         (if (null? selected-repos)
             (error "no direct child native Cairn repositories matched")
             (if (Config-run? config)
                 (run-stack! config workspace-root selected-repos)
                 (print-lines
                  (plan-lines config workspace-root selected-repos snapshots)))))])))

(main)
