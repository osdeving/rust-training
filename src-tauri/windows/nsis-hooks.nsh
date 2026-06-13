!macro NSIS_HOOK_POSTINSTALL
  DetailPrint "Checking Rust toolchain..."

  IfFileExists "$PROFILE\.cargo\bin\rustc.exe" rust_training_rust_found

  nsExec::ExecToStack 'cmd /C "%SystemRoot%\System32\where.exe rustc"'
  Pop $0
  Pop $1
  StrCmp $0 0 rust_training_rust_found rust_training_ask_install

rust_training_ask_install:
  MessageBox MB_YESNO|MB_ICONQUESTION "Rust Training can validate exercises by compiling them with rustc, but Rust was not found on this computer.$\r$\n$\r$\nDo you want to install Rust now using winget?" IDYES rust_training_check_winget IDNO rust_training_done

rust_training_check_winget:
  nsExec::ExecToStack 'cmd /C "%SystemRoot%\System32\where.exe winget"'
  Pop $0
  Pop $1
  StrCmp $0 0 rust_training_install_rust rust_training_no_winget

rust_training_install_rust:
  DetailPrint "Installing Rust via winget..."
  nsExec::ExecToLog 'cmd /C winget install --id Rustlang.Rustup --exact --source winget --accept-package-agreements --accept-source-agreements'
  Pop $0
  StrCmp $0 0 rust_training_install_ok rust_training_install_failed

rust_training_install_ok:
  MessageBox MB_OK|MB_ICONINFORMATION "Rustup was installed. Close and reopen Rust Training so the app can find rustc."
  Goto rust_training_done

rust_training_install_failed:
  MessageBox MB_OK|MB_ICONEXCLAMATION "Rust installation via winget did not finish successfully. You can install it manually at https://rustup.rs and reopen Rust Training."
  ExecShell "open" "https://rustup.rs"
  Goto rust_training_done

rust_training_no_winget:
  MessageBox MB_OK|MB_ICONEXCLAMATION "winget was not found on this Windows installation. Opening https://rustup.rs so you can install Rust manually."
  ExecShell "open" "https://rustup.rs"
  Goto rust_training_done

rust_training_rust_found:
  DetailPrint "Rust toolchain found."

rust_training_done:
!macroend
