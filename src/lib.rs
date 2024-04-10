use neon::prelude::*;
use checksec::elf::CheckSecResults;
use goblin::elf::Elf;
use std::fs;

fn checksec(mut cx: FunctionContext) -> JsResult<JsString> {
    let file_missing = cx.error("could not read file:(")?;
    let could_not_parse = cx.error("could not parse:(")?;
    let could_not_json = cx.error("could not convert to json :(")?;
    let binary = {
        cx.argument::<JsString>(0)?.value(&mut cx).clone()
    };
    if let Ok(buf) = fs::read(&binary) {
        if let Ok(elf) = Elf::parse(&buf) {
            let res = CheckSecResults::parse(&elf);
            if let Ok(json) = serde_json::to_string(&res) {
                return Ok(cx.string(&json));
            } else {
                return cx.throw(could_not_json);
            }
        } else {
            return cx.throw(could_not_parse);
        }
    } else {
        return cx.throw(file_missing);
    }
    
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("checksec", checksec)?;
    Ok(())
}
