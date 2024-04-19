use neon::prelude::*;
use checksec::elf::CheckSecResults;
use goblin::elf::Elf;
use neon::types::buffer::TypedArray;

fn checksec(mut cx: FunctionContext) -> JsResult<JsString> {
    let could_not_parse = cx.error("could not parse:(")?;
    let could_not_json = cx.error("could not convert to json :(")?;

    if let Ok(elf) = Elf::parse(cx.argument::<JsBuffer>(0)?.as_slice(&cx)) {
        let res = CheckSecResults::parse(&elf);
        if let Ok(json) = serde_json::to_string(&res) {
            return Ok(cx.string(&json));
        } else {
            return cx.throw(could_not_json);
        }
    } else {
        return cx.throw(could_not_parse);
    }
}

fn tree_magic(mut cx: FunctionContext) -> JsResult<JsString> {
    let buf = cx.argument::<JsBuffer>(0)?.as_slice(&cx);
    Ok(cx.string(&tree_magic::from_u8(buf)))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("checksec", checksec)?;
    cx.export_function("tree_magic", tree_magic)?;
    Ok(())
}
