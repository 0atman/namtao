notes = (ring :C4, :C4, :C4, :Bb3)
notes_lower = (ring :C3, :C3, :C3, :Bb2)

loop_length = 4
sequence_length = 16

growl = 0.5
hollow = 1
tech_saws = 0
pluck = 0
blade = 0
hats = 0
kick = 0
snare = 0

live_loop :arrangement do
  tech_saws = 0.3
  sleep sequence_length
  pluck = 0.6
  sleep sequence_length
  blade = 0.2
  sleep sequence_length
  
  kick = 1
  pluck = 2
  sleep sequence_length
  hats = 0.5
  sleep sequence_length - 0.5
  snare = 1
end


live_loop :growl do
  use_synth :growl
  
  play notes.tick, sustain: loop_length, amp: growl
  
  sleep loop_length
end

live_loop :hollow do
  use_synth :hollow
  play notes.tick, sustain: loop_length, amp: hollow
  sleep loop_length
end

live_loop :tech_saws do
  use_synth :tech_saws
  play notes_lower.tick, sustain: loop_length, amp: tech_saws
  sleep loop_length
end

live_loop :pluck do
  use_synth :pluck
  with_fx :ping_pong do
    with_fx :lpf, cutoff: 80 do
      play_pattern_timed [:C4, :C4, :C4, :C4, :C4, :C4, :C4, :C4], 0.125, amp: pluck
      play_pattern_timed [:Cs4, :C4, :C4, :C4, :C4, :C4, :C4, :C4], 0.125, amp: pluck
    end
  end
end

live_loop :blade do
  use_synth :prophet
  note = notes_lower.tick
  
  
  play note, sustain: loop_length, amp: blade
  sleep loop_length
  
end

drums = "/home/oatman/.BitwigStudio/installed-packages/1.0/samples/Bitwig/Classic Drum Machines/"
kit = drums + "Legend 505/"

with_fx :ping_pong, room: 1, mix: 0.7 do
  
  live_loop :hats do
    sample kit, "Hat", amp: choose([0.5 * hats, 1 * hats])
    sleep 0.125
  end
  
  live_loop :kick do
    sample kit, "Kick", amp: kick
    sleep 2
  end
  
end

live_loop :snare do
  sleep 1
  sample kit, "Snare", amp: snare
  sleep 1
end