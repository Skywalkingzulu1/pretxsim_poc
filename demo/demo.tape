# PreTxSim demo tape — 35-45s narrative arc, Catppuccin Mocha
# Emits both .mp4 (social feeds) and .gif (GitHub README inline embed).
#
# Run via demo/render.sh  (needs vhs + ttyd + ffmpeg on PATH).
# The tape boots the full stack through demo/run_demo_with_pipe.sh
# (anvil fixture + proxy + answer-stream stdin) inside the recorded shell.
#
# Timeline:
#   00:00-00:10  Boot chain fixture + proxy (air-gapped bind 127.0.0.1:8545)
#   00:10-00:25  The Trap: 7702-delegated target -> gate REFUSES (n)
#   00:25-00:45  The Gate: delegatecall+storage tx -> gate ALLOWS (y)

Output docs/assets/pretxsim_demo.mp4
Output docs/assets/pretxsim_demo.gif

Set Shell "bash"
Set FontSize 15
Set Width 1200
Set Height 720
Set Padding 24
Set Theme "Catppuccin Mocha"
Set TypingSpeed 40ms

# ---------- 00:00-00:10 | Setup ----------
Hide
Type "bash demo/run_demo_with_pipe.sh > /tmp/harness.log 2>&1 &"
Enter
Sleep 4s
Show
Sleep 300ms

Type@60ms "cat /tmp/pretxsim.log"
Enter
Sleep 2000ms

# ---------- 00:10-00:25 | The Trap: delegated EOA ----------
Type@60ms "curl -s -X POST http://127.0.0.1:8545 -d @demo/tx_7702.json &"
Enter
Sleep 4500ms
Type "# EIP-7702 delegation flagged -> refusing broadcast"
Enter
Sleep 800ms
Type "echo n >> /tmp/pretxsim_answers.txt"
Enter
Sleep 2500ms
Ctrl+C

# ---------- 00:25-00:45 | The Gate: risky contract allowed ----------
Type@60ms "curl -s -X POST http://127.0.0.1:8545 -d @demo/tx_risky.json &"
Enter
Sleep 4500ms
Type "# delegatecall + storage writes accepted after review"
Enter
Sleep 800ms
Type "echo y >> /tmp/pretxsim_answers.txt"
Enter
Sleep 2500ms
Type@60ms "tail -1 /tmp/pretxsim.log"
Enter
Sleep 1500ms
