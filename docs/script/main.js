window.onload = function () {
    const search = window.location.hash.split("?").pop()
    if (search) {

    }


}
var initDocsify = function () {
    var configs = [common, customization]
    var docsify = {}
    configs.forEach(function (conf) {
        _.assign(docsify, conf)
    })

    let ver = 'develop'

    const search = window.location.hash.split("?").pop()
    if (search && URLSearchParams) {
        const urlP = new URLSearchParams(search)
        ver = urlP.get("version") || ver
    } else {
        ver = window.localStorage.getItem("version") || 'develop'
    }
    const vTag = document.getElementById('tag_version')
    vTag.textContent = ver
    docsify.basePath = baseUrlGen(ver)
    window.$docsify = docsify
}

var main = function () {
    initDocsify()
}

main()
