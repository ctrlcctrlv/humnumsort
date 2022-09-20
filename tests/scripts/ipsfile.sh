#!/bin/bash
LEN=${LEN:-50}

OCTET1="$(yes 10 | head -n $LEN)"
OCTET2="$(yes $'1\n10' | head -n $LEN | shuf)"
OCTET3="$(for i in $(seq 1 $LEN); do echo 0; done)"
OCTET4="$(yes "`seq 0 9 && seq 200 255 | shuf`" | head -n $LEN | shuf)"

for line in "$(paste <(cat <<< "$OCTET1") <(cat <<< "$OCTET2") <(cat <<< "$OCTET3") <(cat <<< "$OCTET4"))"; do
    awk '{printf "%s.%s.%s.%s\n", $1, $2, $3, $4}' <<< "$line"
done
