use libactionkv::ActionKV;
use std::collections::HashMap;

#[cfg(target_os = "windows")]
const USAGE: &str = "
    akv_mem.exe FILE get KEY
    akv_mem.exe FILE delete KEY
    akv_mem.exe FILE insert KEY VALUE
    akv_mem.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
    akv_mem FILE get KEY
    akv_mem FILE delete KEY
    akv_mem FILE insert KEY VALUE
    akv_mem FILE update KEY VALUE
";

type ByteStr = [u8];
type ByteString = Vec<u8>;

fn store_index_on_disk(a: &mut ActionKV, index_key: &ByteStr) {
    a.index.remove(index_key);
    let index_as_bytes = bincode::serialize(&a.index).unwrap();
    a.index = std::collections::HashMap::new();
    a.insert(index_key, &index_as_bytes).unwrap();
}

fn main() {
    const INDEX_KEY: &ByteStr = b"+index";

    /* Extract the user supplied arguments */
    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE).as_ref();
    let key: &[u8] = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);

    /* Read the specified data store. */
    let path = std::path::Path::new(&fname);
    let mut store = ActionKV::open(path).expect("Unable to open file!");
    store.load().expect("Unable to load data!");

    match action {
        "get" => {
            let index_as_bytes = store.get(&INDEX_KEY).unwrap().unwrap();

            let index_decoded = bincode::deserialize(&index_as_bytes);
            let index: HashMap<ByteString, u64> = index_decoded.unwrap();

            match index.get(key) {
                None => eprintln!("{:?} not found.", String::from_utf8_lossy(&key.to_vec())),
                Some(&i) => {
                    let kv = store.get_at(i).unwrap();
                    println!("{:?}", String::from_utf8_lossy(&kv.value.to_vec()));
                }
            }
        }

        "delete" => store.delete(key).unwrap(),

        "insert" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.insert(key, value).unwrap();
            store_index_on_disk(&mut store, INDEX_KEY);
        }

        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.update(key, value).unwrap();
            store_index_on_disk(&mut store, INDEX_KEY);
        }
        _ => eprintln!("{}", &USAGE),
    }
}
