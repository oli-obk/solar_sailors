var db = null;
var db_name = "quad_indexed_db";
var store_name = "key_value_store";

indexedDB.deleteDatabase(db_name);
var request = indexedDB.open(db_name);
request.onsuccess = function (event) {
    db = request.result;
};
// version change or first creation
request.onupgradeneeded = function (event) {
    var db = event.target.result;

    // Create an objectStore for this database using
    // IDBDatabase.createObjectStore

    var objectStore = db.createObjectStore(store_name, { keyPath: "key" });

    // define what data items the objectStore will contain

    objectStore.createIndex("value", "value", { unique: false });
};

var transaction = null;
var transaction_state = null;

register_plugin = function (importObject) {
    importObject.env.quad_indexed_db_start_transaction = function () {
        if (transaction) {
            // Already running a transaction
            return false;
        } else {
            transaction = db.transaction(store_name, "readwrite").objectStore(store_name);
            transaction.oncomplete = function (event) {
                transaction_state = "success";
            }
            transaction.onerror = function (event) {
                transaction_state = "error";
            }
            transaction.onabort = function (event) {
                transaction_state = "abort";
            }
            return true;
        }
    }
    // need to make atomic transactions, so cache everything and commit at once
    importObject.env.quad_indexed_db_finish_transaction = function (waker) {
        if (transaction) {
            waker = wasm_exports["quad_indexed_db_clone_waker"](waker)
            transaction.oncomplete = function (event) {
                wasm_exports["quad_indexed_db_wake"](waker)
                transaction_state = "success";
            }
            transaction = null;
        }
        return transaction_state == "success";
    }
    importObject.env.quad_indexed_db_get = function (key) {
        var request = db.transaction(store_name).objectStore(store_name).get(get_js_object(key));
        result = { 'done': 0 };
        request.onsuccess = function (event) {
            if (request.result) {
                result.value = request.result.value;
            }
            if (request.waker) {
                wasm_exports["quad_indexed_db_wake"](waker)
            }
            result.done = 1;
        };
        return js_object(result);
    }
    importObject.env.quad_indexed_db_set = function (key, value) {
        var key = get_js_object(key);
        var value = get_js_object(value);
        var req = transaction.put({ key, value });
    }
    importObject.env.quad_indexed_db_remove = function (key) {
        transaction.remove(get_js_object(key));
    }
    importObject.env.quad_indexed_db_clear = function () {
        indexedDB.deleteDatabase(db_name);
    }
}

miniquad_add_plugin({
    register_plugin,
    name: "quad_indexed_db",
    version: "0.1.0"
});
