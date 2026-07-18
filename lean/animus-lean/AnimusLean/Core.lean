namespace AnimusLean

/-! Pure Lean loop-fabric kernel. This module intentionally imports no IO or host-effect module. -/

public inductive DecisionKind where
  | model
  | tool
  | respond
  | stop
  deriving Repr, DecidableEq

public structure ToolArgument where
  name : String
  value : String
  deriving Repr, DecidableEq

public structure ToolRequest where
  name : String
  schemaIdentity : String
  arguments : Array ToolArgument
  correlationIdentity : String
  visibleOutputBudget : Nat
  mutationTargets : Array String
  requiredCapabilities : Array String
  approvalPosture : String
  deriving Repr, DecidableEq

public inductive Decision where
  | model (prompt : String)
  | tool (request : ToolRequest)
  | respond (message : String)
  | stop
  deriving Repr, DecidableEq

public inductive Phase where
  | initial
  | waitingModel
  | waitingTool
  | terminal
  deriving Repr, DecidableEq

public structure Budget where
  remainingSteps : Nat
  remainingPayloadBytes : Nat
  deriving Repr, DecidableEq

public structure FabricState where
  phase : Phase
  budget : Budget
  deriving Repr, DecidableEq

public structure Turn where
  sequence : Nat
  correlationIdentity : String
  summary : String
  lastModelMessage : Option String
  lastToolOutput : Option String
  deriving Repr, DecidableEq

public structure Profile where
  maxTextBytes : Nat
  maxArguments : Nat
  maxTargets : Nat
  maxCapabilities : Nat
  deriving Repr, DecidableEq

public inductive FabricError where
  | invalidProfile
  | invalidTurn
  | invalidDecision
  | staleCorrelation
  | budgetExhausted
  | terminalState
  deriving Repr, DecidableEq

public def Decision.kind : Decision → DecisionKind
  | .model _ => .model
  | .tool _ => .tool
  | .respond _ => .respond
  | .stop => .stop

public def Profile.valid (profile : Profile) : Bool :=
  profile.maxTextBytes > 0 &&
  profile.maxArguments > 0 &&
  profile.maxTargets > 0 &&
  profile.maxCapabilities > 0

public def validText (profile : Profile) (text : String) : Bool :=
  !text.trimAscii.isEmpty && text.utf8ByteSize ≤ profile.maxTextBytes

public def validLabel (profile : Profile) (text : String) : Bool :=
  validText profile text && !text.any (fun character => character = '\u0000')

public def validateTool (profile : Profile) (request : ToolRequest) : Except FabricError Unit := do
  if !validLabel profile request.name then throw .invalidDecision
  if !validLabel profile request.schemaIdentity then throw .invalidDecision
  if !validLabel profile request.correlationIdentity then throw .invalidDecision
  if request.visibleOutputBudget = 0 then throw .invalidDecision
  if request.arguments.isEmpty || request.arguments.size > profile.maxArguments then
    throw .invalidDecision
  if request.mutationTargets.size > profile.maxTargets then throw .invalidDecision
  if request.requiredCapabilities.isEmpty ||
      request.requiredCapabilities.size > profile.maxCapabilities then
    throw .invalidDecision
  if request.approvalPosture != "pre_approved" &&
      request.approvalPosture != "requires_confirmation" then
    throw .invalidDecision
  for argument in request.arguments do
    if !validLabel profile argument.name || !validText profile argument.value then
      throw .invalidDecision
  for target in request.mutationTargets do
    if !validLabel profile target then throw .invalidDecision
  for capability in request.requiredCapabilities do
    if !validLabel profile capability then throw .invalidDecision

public def validateDecision (profile : Profile) : Decision → Except FabricError Unit
  | .model prompt => if validText profile prompt then .ok () else .error .invalidDecision
  | .tool request => validateTool profile request
  | .respond message => if validText profile message then .ok () else .error .invalidDecision
  | .stop => .ok ()

public def consumeStep (budget : Budget) : Budget :=
  { budget with remainingSteps := budget.remainingSteps - 1 }

public def step
    (profile : Profile)
    (state : FabricState)
    (turn : Turn) : Except FabricError (FabricState × Decision) :=
  if !profile.valid then .error .invalidProfile
  else if state.phase = .terminal then .error .terminalState
  else if state.budget.remainingSteps = 0 then .error .budgetExhausted
  else if turn.sequence = 0 || !validLabel profile turn.correlationIdentity ||
      !validText profile turn.summary then .error .invalidTurn
  else
    let nextBudget := consumeStep state.budget
    match turn.lastToolOutput, turn.lastModelMessage, state.phase with
    | some output, _, _ =>
        if validText profile output then
          .ok ({ phase := .terminal, budget := nextBudget }, .respond output)
        else
          .error .invalidTurn
    | none, some message, _ =>
        if validText profile message then
          .ok ({ phase := .terminal, budget := nextBudget }, .respond message)
        else
          .error .invalidTurn
    | none, none, .initial =>
        .ok ({ phase := .waitingModel, budget := nextBudget }, .model turn.summary)
    | none, none, _ => .error .invalidTurn

deriving instance BEq for DecisionKind
deriving instance BEq for ToolArgument
deriving instance BEq for ToolRequest
deriving instance BEq for Decision
deriving instance BEq for Phase
deriving instance BEq for Budget
deriving instance BEq for FabricState
deriving instance BEq for Turn
deriving instance BEq for Profile
deriving instance BEq for FabricError

/-! Reviewed theorem surface for the encoded kernel. -/

public theorem deterministic_transition
    (profile : Profile) (state : FabricState) (turn : Turn) :
    step profile state turn = step profile state turn := rfl

public theorem closed_decision_set (decision : Decision) :
    decision.kind = .model ∨ decision.kind = .tool ∨
    decision.kind = .respond ∨ decision.kind = .stop := by
  cases decision <;> simp [Decision.kind]

public theorem consume_step_nonincreasing (budget : Budget) :
    (consumeStep budget).remainingSteps ≤ budget.remainingSteps ∧
    (consumeStep budget).remainingPayloadBytes ≤ budget.remainingPayloadBytes := by
  simp [consumeStep]

public theorem terminal_absorption
    (profile : Profile) (budget : Budget) (turn : Turn) :
    ∃ error, step profile { phase := .terminal, budget := budget } turn = .error error := by
  cases valid : profile.valid with
  | false => exact ⟨.invalidProfile, by simp [step, valid]⟩
  | true => exact ⟨.terminalState, by simp [step, valid]⟩

public theorem invalid_profile_fails_closed
    (profile : Profile) (state : FabricState) (turn : Turn)
    (invalid : profile.valid = false) :
    step profile state turn = .error .invalidProfile := by
  simp [step, invalid]

public theorem reference_initial_phase
    (profile : Profile) (budget : Budget) (turn : Turn)
    (profileValid : profile.valid = true)
    (steps : budget.remainingSteps ≠ 0)
    (sequence : turn.sequence ≠ 0)
    (correlation : validLabel profile turn.correlationIdentity = true)
    (summary : validText profile turn.summary = true)
    (noModel : turn.lastModelMessage = none)
    (noTool : turn.lastToolOutput = none) :
    step profile { phase := .initial, budget := budget } turn =
      .ok ({ phase := .waitingModel, budget := consumeStep budget }, .model turn.summary) := by
  simp [step, profileValid, steps, sequence, correlation, summary, noModel, noTool] <;> rfl

public theorem reference_model_then_respond
    (profile : Profile) (budget : Budget) (turn : Turn) (message : String)
    (profileValid : profile.valid = true)
    (steps : budget.remainingSteps ≠ 0)
    (sequence : turn.sequence ≠ 0)
    (correlation : validLabel profile turn.correlationIdentity = true)
    (summary : validText profile turn.summary = true)
    (messageValid : validText profile message = true)
    (noTool : turn.lastToolOutput = none)
    (hasModel : turn.lastModelMessage = some message) :
    step profile { phase := .waitingModel, budget := budget } turn =
      .ok ({ phase := .terminal, budget := consumeStep budget }, .respond message) := by
  simp [step, profileValid, steps, sequence, correlation, summary, messageValid, noTool, hasModel] <;> rfl

end AnimusLean
