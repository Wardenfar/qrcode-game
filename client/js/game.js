export function get_game_toml_val() {
    return global.game_toml_val;
}

export function on_hash_changed(closure) {
    window.onpopstate = function (event) {
        if (window.hash_changed === true) {
            window.hash_changed = false
        } else {
            closure(window.location.hash)
        }
    }
}

export function set_hash(hash) {
    window.hash_changed = true
    window.location = "#" + hash;
}