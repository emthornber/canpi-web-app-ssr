#! /bin/bash
################################################################################
#
#	canpi-ssrctl.bash
#
#   Script to control canpi-ssr
#
#   04 August, 2025 - E M Thornber
#   Created (from canpidctl.bash)
#
################################################################################

PID_FILE=/run/canpi-ssr.pid

start_canpi-ssr() {
    pid=`pgrep --exact canpi-ssr`
    if [ $? -eq 0 ] ;
    then
        echo canpi-ssr already running
    else
        echo starting canpi-ssr
        /usr/local/bin/canpi-ssr >> "/var/log/canpi-ssr/stdout.log" 2>>"/var/log/canpi-ssr/stderr.log" &
        echo $! > $PID_FILE
        if [ ! `pgrep --exact canpi-ssr` ] ; then echo canpi-ssr did not start ; fi
    fi
}

stop_canpi-ssr() {
    if [ `pgrep --exact canpi-ssr` ]; then
        pkill -9 --exact canpi-ssr
        echo canpi-ssr killed
    else
        echo canpi-ssr not active
    fi
}

# main code
case "$1" in
start)
    start_canpi-ssr
    ;;
restart)
    $0 stop
    $0 start
    ;;
stop)
    stop_canpi-ssr
    ;;
*)
    echo "Usage: $0 (start|restart|stop)"
    exit 1
    ;;
esac

exit 0
