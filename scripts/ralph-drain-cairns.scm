;; Ralph Cairn drain loop for the Animus workspace.
;;
;; Default mode prints a deterministic drain plan. --run directly launches the
;; configured Pi command (default: pi -p) for each selected Cairn change using
;; Steel's process API. This Animus variant defaults to the repo-pinned Cairn app:
;; nix run .#cairn -- ... --root .

(require "steel/result")
(require-builtin steel/process)

(define default-root ".")
(define default-limit 1)
(define all-rounds-limit 0)
(define first-round-index 1)
(define failing-status-probe 7)
(define sample-limit-value 2)
(define default-agent-command "pi -p")
(define default-cairn-command "nix run .#cairn --")
(define path-separator "/")
(define process-cd-script "cd \"$1\" && shift && exec \"$@\"")
(define process-shell-name "sh")

(struct Config
  (root limit agent-command cairn-command emit-prompts? emit-commands? run? changes self-test? help?))

(struct Change (name path))

(define (say value)
  (display value)
  (newline))

(define (string-prefix? prefix text)
  (let ([prefix-length (string-length prefix)] [text-length (string-length text)])
    (and (<= prefix-length text-length)
         (string=? prefix (substring text 0 prefix-length)))))

(define (space? value)
  (char=? value #\space))

(define (split-words text)
  (let ([text-length (string-length text)])
    (let loop ([index 0] [token-start #false] [tokens '()])
      (cond
        [(= index text-length)
         (reverse
          (if token-start
              (cons (substring text token-start text-length) tokens)
              tokens))]
        [(space? (string-ref text index))
         (if token-start
             (loop (+ index 1) #false (cons (substring text token-start index) tokens))
             (loop (+ index 1) #false tokens))]
        [else
         (if token-start
             (loop (+ index 1) token-start tokens)
             (loop (+ index 1) index tokens))]))))

(define (ends-with-slash? text)
  (let ([text-length (string-length text)])
    (and (> text-length 0) (char=? (string-ref text (- text-length 1)) #\/))))

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
    (if (and parsed (>= parsed all-rounds-limit)) parsed #false)))

(define (usage-lines)
  (list
   "Usage: steel scripts/ralph-drain-cairns.scm [options]"
   ""
   "Options:"
   "  --root PATH        Animus repo root containing cairn/changes (default: .)"
   "  --limit N          Number of changes to include; 0 means all (default: 1)"
   "  --change NAME      Include one named change; repeat for multiple changes"
   "  --agent COMMAND    Pi command used by --run (default: pi -p)"
   "  --cairn COMMAND    Cairn CLI command prefix used in prompts"
   "                    (default: nix run .#cairn --)"
   "  --run              Launch the Pi command for each selected change"
   "  --no-prompts       Omit Ralph prompt heredocs in plan mode"
   "  --no-commands      Omit shell command blocks in plan mode"
   "  --self-test        Run positive and negative script self-tests"
   "  --help             Show this help"
   ""
   "Examples:"
   "  steel scripts/ralph-drain-cairns.scm --limit 3"
   "  steel scripts/ralph-drain-cairns.scm --change introduce-steel-runtime-loop"
   "  steel scripts/ralph-drain-cairns.scm --run --limit 1"
   "  steel scripts/ralph-drain-cairns.scm --run --limit 0 --agent \"pi -p --model sonnet:high\""))

(define (parse-args raw-args)
  (let loop ([args raw-args]
             [root default-root]
             [limit default-limit]
             [agent-command default-agent-command]
             [cairn-command default-cairn-command]
             [emit-prompts? #true]
             [emit-commands? #true]
             [run? #false]
             [changes '()]
             [self-test? #false]
             [help? #false])
    (cond
      [(null? args)
       (Config root limit agent-command cairn-command emit-prompts? emit-commands? run? (reverse changes) self-test? help?)]
      [(string=? (car args) "--root")
       (if (null? (cdr args))
           (error "--root requires a path")
           (loop (cddr args) (cadr args) limit agent-command cairn-command emit-prompts? emit-commands? run? changes self-test? help?))]
      [(string-prefix? "--root=" (car args))
       (loop (cdr args) (substring (car args) (string-length "--root=") (string-length (car args))) limit agent-command cairn-command emit-prompts? emit-commands? run? changes self-test? help?)]
      [(string=? (car args) "--limit")
       (if (null? (cdr args))
           (error "--limit requires a non-negative number")
           (let ([parsed-limit (parse-limit-value (cadr args))])
             (if parsed-limit
                 (loop (cddr args) root parsed-limit agent-command cairn-command emit-prompts? emit-commands? run? changes self-test? help?)
                 (error "--limit requires a non-negative number"))))]
      [(string-prefix? "--limit=" (car args))
       (let ([parsed-limit (parse-limit-value (substring (car args) (string-length "--limit=") (string-length (car args))))])
         (if parsed-limit
             (loop (cdr args) root parsed-limit agent-command cairn-command emit-prompts? emit-commands? run? changes self-test? help?)
             (error "--limit requires a non-negative number")))]
      [(string=? (car args) "--change")
       (if (null? (cdr args))
           (error "--change requires a change name")
           (loop (cddr args) root limit agent-command cairn-command emit-prompts? emit-commands? run? (cons (cadr args) changes) self-test? help?))]
      [(string-prefix? "--change=" (car args))
       (loop (cdr args) root limit agent-command cairn-command emit-prompts? emit-commands? run? (cons (substring (car args) (string-length "--change=") (string-length (car args))) changes) self-test? help?)]
      [(string=? (car args) "--agent")
       (if (null? (cdr args))
           (error "--agent requires a command")
           (loop (cddr args) root limit (cadr args) cairn-command emit-prompts? emit-commands? run? changes self-test? help?))]
      [(string-prefix? "--agent=" (car args))
       (loop (cdr args) root limit (substring (car args) (string-length "--agent=") (string-length (car args))) cairn-command emit-prompts? emit-commands? run? changes self-test? help?)]
      [(string=? (car args) "--cairn")
       (if (null? (cdr args))
           (error "--cairn requires a command")
           (loop (cddr args) root limit agent-command (cadr args) emit-prompts? emit-commands? run? changes self-test? help?))]
      [(string-prefix? "--cairn=" (car args))
       (loop (cdr args) root limit agent-command (substring (car args) (string-length "--cairn=") (string-length (car args))) emit-prompts? emit-commands? run? changes self-test? help?)]
      [(string=? (car args) "--run")
       (loop (cdr args) root limit agent-command cairn-command emit-prompts? emit-commands? #true changes self-test? help?)]
      [(string=? (car args) "--no-prompts")
       (loop (cdr args) root limit agent-command cairn-command #false emit-commands? run? changes self-test? help?)]
      [(string=? (car args) "--no-commands")
       (loop (cdr args) root limit agent-command cairn-command emit-prompts? #false run? changes self-test? help?)]
      [(string=? (car args) "--self-test")
       (loop (cdr args) root limit agent-command cairn-command emit-prompts? emit-commands? run? changes #true help?)]
      [(string=? (car args) "--help")
       (loop (cdr args) root limit agent-command cairn-command emit-prompts? emit-commands? run? changes self-test? #true)]
      [else (error (string-append "unknown argument: " (car args)))])))

(define (collect-change-paths changes-root)
  (let ([iter (read-dir-iter changes-root)])
    (let loop ([paths '()])
      (let ([entry (read-dir-iter-next! iter)])
        (cond
          [(not entry) (sort paths string<?)]
          [(read-dir-entry-is-dir? entry) (loop (cons (read-dir-entry-path entry) paths))]
          [else (loop paths)])))))

(define (change-path->change path)
  (Change (basename path) path))

(define (discover-changes root)
  (map change-path->change (collect-change-paths (path-join (path-join root "cairn") "changes"))))

(define (change-selected? selected-names change)
  (or (null? selected-names) (member (Change-name change) selected-names)))

(define (filter-changes changes selected-names)
  (cond
    [(null? changes) '()]
    [(change-selected? selected-names (car changes))
     (cons (car changes) (filter-changes (cdr changes) selected-names))]
    [else (filter-changes (cdr changes) selected-names)]))

(define (take-up-to limit items)
  (cond
    [(<= limit all-rounds-limit) '()]
    [(null? items) '()]
    [else (cons (car items) (take-up-to (- limit 1) (cdr items)))]))

(define (select-changes changes config)
  (let ([filtered-changes (filter-changes changes (Config-changes config))]
        [limit (Config-limit config)])
    (if (= limit all-rounds-limit)
        filtered-changes
        (take-up-to limit filtered-changes))))

(define (change-artifact-lines change)
  (list
   (string-append "- " (path-join (Change-path change) "proposal.md"))
   (string-append "- " (path-join (Change-path change) "design.md"))
   (string-append "- " (path-join (Change-path change) "tasks.md"))
   (string-append "- " (path-join (path-join (Change-path change) "specs") "**/spec.md"))))

(define (gate-command config gate change)
  (string-append (Config-cairn-command config) " gate " gate " " (Change-name change) " --root " (Config-root config)))

(define (cairn-command config command change)
  (string-append (Config-cairn-command config) " " command " " (Change-name change) " --root " (Config-root config)))

(define (animus-repo-guidance-lines config)
  (list
   (string-append "Use the repo-pinned Cairn command from the Animus root: `" (Config-cairn-command config) " ... --root " (Config-root config) "`; do not swap in a newer sibling Cairn binary unless the change requires it.")
   "Read `README.md`, `docs/safety-contract.md`, `docs/request-lifecycle.md`, `docs/runtime-facade-inventory.md`, and relevant change specs before editing."
   "Run `scripts/cairn/check-roadmap --root .` and avoid draining changes blocked by active prerequisites unless the dependency is completed in the same batch."
   "Keep runtime logic split into pure functional cores and thin imperative shells; reusable crates must not gain ambient filesystem, process, network, credential, clock, terminal, or persistence authority."
   "For focused rails, keep positive and negative fixtures together and wire deterministic, credential-free rails into named Nix checks when stable."
   "For Cairn evidence, prefer safe tracked evidence in the change/archive package; run `scripts/cairn/check-evidence-safety --root .` when adding `evidence.md`."
   "After sync, search accepted specs for new `r[...]` IDs; if sync omitted the delta, manually merge the accepted spec text and re-run validation."
   "Preserve non-claims unless a separate accepted aggregate gate promotes them: no live-provider, TUI/daemon dogfood, VM, plugin-runtime, performance, or production-readiness claims by default."))

(define (ralph-prompt-lines config change)
  (append
   (list
    (string-append "Goal: drain Cairn change `" (Change-name change) "` in the Animus repo `" (Config-root config) "` end to end.")
    ""
    "Success criteria: implementation is complete, tasks are checked off only after verification, accepted specs are synced, the change archives successfully, validation passes, reviewable evidence stays safe and tracked when needed, and a why-focused commit is created. Do not push unless explicitly asked."
    ""
    "Constraints: preserve user changes; use the repo-pinned Cairn CLI; run gates before implementation; run the smallest relevant baseline before core logic edits; use functional-core / imperative-shell decomposition; keep Tiger Style constraints; do not parallelize lifecycle mutations; stop and report a concrete blocker instead of guessing."
    ""
    "Animus-specific guardrails:")
   (map (lambda (line) (string-append "- " line)) (animus-repo-guidance-lines config))
   (list
    ""
    "Read these change artifacts first:")
   (change-artifact-lines change)
   (list
    ""
    "Ralph loop:"
    "1. Resolve: inspect proposal/design/tasks/spec deltas and extract requirement IDs, dependencies, acceptance criteria, affected crates/docs, and relevant checks."
    "2. Assess: run proposal/design/tasks gates, repo validation, and roadmap validation; record pre-existing failures exactly."
    "3. Loop: implement one smallest safe task slice, then run the smallest relevant check before marking that task done."
    "4. Perform: after all tasks are complete, dry-run sync/archive, inspect plans, then execute sync/archive only if unblocked."
    "5. Harvest: validate, verify accepted spec text landed, run evidence safety when evidence changed, commit the completed archive, and report completed archive path, checks, commit, blockers, and remaining active count."
    ""
    "Blockers: leave the change active if gates fail for unrelated reasons, dependencies are incomplete, requirements conflict, credentials/external systems are needed, tests fail before edits and block reliable verification, evidence cannot be kept safe/tracked, or sync/archive dry-runs are blocked.")))

(define (ralph-prompt config change)
  (join-lines (ralph-prompt-lines config change)))

(define (command-lines config change)
  (list
   "```sh"
   (string-append "cd " (Config-root config))
   (string-append (Config-cairn-command config) " validate --root " (Config-root config))
   "scripts/cairn/check-roadmap --root ."
   (gate-command config "proposal" change)
   (gate-command config "design" change)
   (gate-command config "tasks" change)
   (string-append (Config-agent-command config) " --name \"Animus Cairn drain: " (Change-name change) "\" '<generated prompt>'")
   (cairn-command config "sync" change)
   "# Inspect the sync dry-run before executing:"
   (string-append (Config-cairn-command config) " sync " (Change-name change) " --root " (Config-root config) " --execute")
   (string-append (Config-cairn-command config) " validate --root " (Config-root config))
   (cairn-command config "archive" change)
   "# Inspect the archive dry-run before executing:"
   (string-append "CAIRN_ARCHIVE_DATE=$(date +%F) " (Config-cairn-command config) " archive " (Change-name change) " --root " (Config-root config) " --execute")
   (string-append (Config-cairn-command config) " validate --root " (Config-root config))
   "scripts/cairn/check-evidence-safety --root ."
   "```"))

(define (round-lines config round-index change)
  (append
   (list
    ""
    (string-append "## Round " (number->string round-index) ": " (Change-name change))
    ""
    "### Resolve"
    "Artifacts to read:")
   (change-artifact-lines change)
   (list
    ""
    "### Assess"
    (string-append "Run roadmap validation and gates for `" (Change-name change) "` with the repo-pinned Cairn app; skip it if roadmap reports active prerequisite blockers."))
   (if (Config-emit-commands? config)
       (append (list "" "### Commands") (command-lines config change))
       '())
   (if (Config-emit-prompts? config)
       (append
        (list "" "### Ralph prompt" "```text")
        (ralph-prompt-lines config change)
        (list "```"))
       '())
   (list
    ""
    "### Harvest"
    "Report completed archive path, checks run, commit hash/message, blockers if any, preserved user changes, and remaining active changes.")))

(define (plan-lines config all-changes selected-changes)
  (append
   (list
    "# Animus Ralph Cairn Drain Loop"
    ""
    (string-append "Root: `" (Config-root config) "`")
    (string-append "Cairn command: `" (Config-cairn-command config) "`")
    (string-append "Discovered active changes: " (number->string (length all-changes)))
    (string-append "Selected rounds: " (number->string (length selected-changes)))
    ""
    "Default mode only prints this plan. Re-run with `--run` to launch `pi -p` for each selected change. Use `--change` for unblocked roadmap candidates.")
   (let loop ([round-index first-round-index] [remaining selected-changes] [lines '()])
     (if (null? remaining)
         lines
         (loop (+ round-index 1) (cdr remaining) (append lines (round-lines config round-index (car remaining))))))))

(define (external-command-status root program args)
  (unwrap-ok
   (process-wait
    (unwrap-ok
     (spawn-process
      (command process-shell-name
               (append
                (list "-c" process-cd-script process-shell-name root program)
                args)))))))

(define (agent-command-parts config)
  (let ([parts (split-words (Config-agent-command config))])
    (if (null? parts)
        (error "--agent must not be empty")
        parts)))

(define (run-pi-change! config round-index change)
  (let* ([parts (agent-command-parts config)]
         [program (car parts)]
         [base-args (cdr parts)]
         [session-name (string-append "Animus Cairn drain: " (Change-name change))]
         [args (append base-args (list "--name" session-name (ralph-prompt config change)))])
    (say (string-append "==> Pi Animus Cairn round " (number->string round-index) ": " (Change-name change)))
    (let ([status (external-command-status (Config-root config) program args)])
      (if (= status 0)
          (say (string-append "<== completed: " (Change-name change)))
          (error (string-append "pi command failed for " (Change-name change) " with exit " (number->string status)))))))

(define (run-selected-changes! config selected-changes)
  (say "Animus Ralph Cairn drain: launching Pi sequentially")
  (say (string-append "Root: " (Config-root config)))
  (let loop ([round-index first-round-index] [remaining selected-changes])
    (if (null? remaining)
        (say "Animus Ralph Cairn drain complete")
        (begin
          (run-pi-change! config round-index (car remaining))
          (loop (+ round-index 1) (cdr remaining))))))

(define (print-lines lines)
  (for-each say lines))

(define (check label condition)
  (if condition
      (say (string-append "ok - " label))
      (error (string-append "self-test failed: " label))))

(define (run-self-test)
  (let* ([first-change (Change "alpha" "/tmp/repo/cairn/changes/alpha")]
         [second-change (Change "beta" "/tmp/repo/cairn/changes/beta")]
         [sample-config (Config "/tmp/repo" 1 default-agent-command default-cairn-command #true #true #false '() #false #false)]
         [filtered-config (Config "/tmp/repo" all-rounds-limit default-agent-command default-cairn-command #true #true #false '("beta") #false #false)]
         [sample-plan (plan-lines sample-config (list first-change second-change) (list first-change))]
         [sample-prompt (ralph-prompt sample-config first-change)])
    (check "prefix positive" (string-prefix? "--root=" "--root=/tmp/repo"))
    (check "prefix negative" (not (string-prefix? "--root=" "--roo=/tmp/repo")))
    (check "basename positive" (string=? (basename "/tmp/repo/cairn/changes/alpha") "alpha"))
    (check "split agent command" (equal? (split-words "pi -p --model sonnet:high") (list "pi" "-p" "--model" "sonnet:high")))
    (check "limit parse positive" (= (parse-limit-value (number->string sample-limit-value)) sample-limit-value))
    (check "limit parse negative" (not (parse-limit-value "nope")))
    (check "select limit" (= (length (select-changes (list first-change second-change) sample-config)) 1))
    (check "select named change" (string=? (Change-name (car (select-changes (list first-change second-change) filtered-config))) "beta"))
    (check "default cairn command is repo pinned" (string=? (Config-cairn-command sample-config) default-cairn-command))
    (check "plan names repo pinned cairn command" (member (string-append "Cairn command: `" default-cairn-command "`") sample-plan))
    (check "prompt includes Animus guidance" (string-prefix? "Goal: drain Cairn change" sample-prompt))
    (check "prompt names evidence safety" (member "- For Cairn evidence, prefer safe tracked evidence in the change/archive package; run `scripts/cairn/check-evidence-safety --root .` when adding `evidence.md`." (ralph-prompt-lines sample-config first-change)))
    (check "external command success" (= (external-command-status "." "true" '()) 0))
    (check "external command failure" (= (external-command-status "." "sh" (list "-c" (string-append "exit " (number->string failing-status-probe)))) failing-status-probe))
    (say "self-test complete")))

(define (script-args)
  pi-args)

(define (main)
  (let ([config (parse-args (script-args))])
    (cond
      [(Config-help? config) (print-lines (usage-lines))]
      [(Config-self-test? config) (run-self-test)]
      [else
       (let* ([all-changes (discover-changes (Config-root config))]
              [selected-changes (select-changes all-changes config)])
         (cond
           [(null? selected-changes)
            (print-lines
             (list
              "# Animus Ralph Cairn Drain Loop"
              ""
              (string-append "Root: `" (Config-root config) "`")
              (string-append "Cairn command: `" (Config-cairn-command config) "`")
              (string-append "Discovered active changes: " (number->string (length all-changes)))
              "Selected rounds: 0"
              ""
              "No active changes matched the requested selection."))]
           [(Config-run? config) (run-selected-changes! config selected-changes)]
           [else (print-lines (plan-lines config all-changes selected-changes))]))])))

(main)
