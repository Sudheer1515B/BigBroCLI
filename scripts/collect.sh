#!/bin/sh


while :; do
    mem_total = 0
    mem_available = 0
    swap_total = 0
    swap_free = 0

    #get the info above
    while read -r key value unit; do
        case "$key" in
            MemTotal:) mem_total = $value ;;
            MemAvailable:) mem_available = $value;;
            SwapTotal:) swap_total = $value;;
            SwapFree:) swap_free = $value;;
        esac
    done < /proc/meminfo

    #we also gotta stop if the system don't procide valid mem data

    [ "$mem_total" -gt 0 ] || exit 1

    printf '%s %s %s %s' \
       "$mem_total" "$mem_available" \
       "$swap_total" "$swap_free"

    #readin the overall and indiv CPU cores
    while read -r core user nice system idle iowait irq softirq steal rest; do
        case "$core" in
            cpu|cpu[0-9]*)
                total = $((user + nice + system + idle + iowait + irq + softirq +steal))
                idle_total = $((idle + iowait))

                printf '%s:%s:%s' \
                  "$core" "$total" "$idle_total"
                ;;
        esac
    done < /proc/stat

    printf '\n'
    sleep 1
done
