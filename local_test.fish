set score 0
set now 0
cargo run --release --bin ahc006-a < ./tools/in/0000.txt > out.txt ^tmp
cat tmp | while read line
    set now $line
end
echo 0 $now
set score (math $score + $now)

for val in (seq 1 9)
  cargo run --release --bin ahc006-a < ./tools/in/000$val.txt >./out/000$val.txt ^tmp
  cat tmp | while read line
    set now $line
  end
  echo $val $now
  set score (math $score + $now)
end

for val in (seq 10 99)
  cargo run --release --bin ahc006-a < ./tools/in/00$val.txt >./out/00$val.txt ^tmp
  cat tmp | while read line
    set now $line
  end
  echo $val $now
  set score (math $score + $now)
end
echo score: $score
rm tmp