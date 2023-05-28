let $sourceType = $("input[name='sourceType']")
let $pattern = $("#pattern")
let $srcTxt = $("#srcTxt")
let $srcFile = $("#srcFile")
let $textContainer = $("#textContainer")
let $caseCheck = $("#caseCheck")
let $findBtn = $("#findBtn")
let $clearBtn = $("#clearBtn")
let $matches = $("#matches")
let $fileError = $("#fileError")

const findMatch = async () => {
    let src = await retrieveContent()
    if(!src) {
        $fileError.prop("hidden", false)
        return
    }

    $.ajax({
        url: "/",
        crossDomain: true,
        type: "POST",
        dataType : "json",
        contentType: "application/json",
        data: prepareReqData(src),
    })
    .done((res) => renderResult(res, src, $caseCheck.is(":checked")))
    .fail(() => { return }) //don't do this kind of things in real-world apps...
}

let lastFile = undefined
let lastSrc = null
const retrieveContent = async () => {
    let src = { // giving it a default value
        text: "",
        pattern: "",
    };

    src.pattern = $pattern.val()
    switch($sourceType.filter(":checked").val()) {
        case "file":
            let file = $srcFile[0].files[0]
            if(file.type !== "text/plain") return

            if(lastFile === file) {
                if(lastSrc.pattern != src.pattern)
                    lastSrc.pattern = src.pattern
                return lastSrc
            }
            lastFile = file

            src.text = await new Blob([file], { type: file.type }).text()
            break;
        case "text":
            src.text = $srcTxt.val()
            break;
    }
    lastSrc = src

    return src
}

const resolveSelector = () => {
    restartState()
    switch($sourceType.filter(":checked").val()) {
        case "file":
            $srcTxt.prop("hidden", true)
            $srcFile.prop("hidden", false)
            break;
        case "text":
            $srcTxt.prop("hidden", false)
            $srcFile.prop("hidden", true)
            break;
    }
}

const prepareReqData = (src) => {
    let res = src;
    if(!$caseCheck.is(":checked")){
        res = { ...src }
        res.text = res.text.toLowerCase()
        res.pattern = res.pattern.toLowerCase()
    }
    return JSON.stringify(res)
}

const restartState = () => {
    $pattern.val("")
    $srcTxt.val("")
    $srcFile.val("")
    $textContainer.empty()
    $caseCheck.prop("checked", true)
    $matches.prop("hidden", true)
    $fileError.prop("hidden", true)
    $findBtn.prop("disabled", true)
}

const renderResult = (res, src, isCs) => {
    let rgx = new RegExp(`(${src.pattern})`, !isCs ? "ig" : "g")
    $textContainer.html(src.text.replace(rgx, "<mark>$1</mark>"))

    $matches.prop("hidden", false)
    $matches.html(`${res.ocurrences} match${res.ocurrences != 1 ? "es" : ""} found`)
}

const changeFindBtnState = () => {
    let cond = $pattern.val() !== "" &&
        ($srcFile[0]?.files[0] || $srcTxt.val().length !== 0)
    $findBtn.prop("disabled", cond ? false : true)
}

$sourceType.on("change", resolveSelector)
$findBtn.on("click", findMatch)
$clearBtn.on("click", restartState)
$pattern.on("input", changeFindBtnState)
$srcTxt.on("input", changeFindBtnState)
$srcFile.on("change", changeFindBtnState)
