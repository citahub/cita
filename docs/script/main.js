var initDocsify = function () {
    var configs = [common,customization]
    var docsify = {}
    configs.forEach(function (conf) {
        _.assign(docsify, conf)
    })
    window.$docsify = docsify
}

var main = function () {
    initDocsify()
}

main()
