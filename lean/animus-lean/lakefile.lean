import Lake
open Lake DSL

package «animus-lean» where
  version := v!"0.1.0"

lean_lib AnimusLean where
  srcDir := "."

lean_exe animus_lean where
  root := `Main

lean_exe animus_lean_tests where
  root := `Test
