function color_msg() {
    local color=${1:?'(r)ed or (g)reen (b)lue (y)ellow (p)urple (c)yan'}

    if (( 2 > $# )); then
        return
    fi

    if [[ 'r' == $color ]]; then
        echo -e '\033[31m'${@:2}'\033[0m' # red
    elif [[ 'g' == $color ]]; then
        echo -e '\033[32m'${@:2}'\033[0m' # green
    elif [[ 'b' == $color ]]; then
        echo -e '\033[34m'${@:2}'\033[0m' # blue
    elif [[ 'y' == $color ]]; then
        echo -e '\033[33m'${@:2}'\033[0m' # yellow
    elif [[ 'p' == $color ]]; then
        echo -e '\033[35m'${@:2}'\033[0m' # purple
    elif [[ 'c' == $color ]]; then
        echo -e '\033[36m'${@:2}'\033[0m' # cyan
    else
        echo -e '\033[37m'${@:2}'\033[0m' # white
    fi
}
