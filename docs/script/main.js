const lngs = {
    'zh-CN': "中文",
    'en-US': 'English'
}
var initDocsify = function () {

    var docsify = { ...common,
        ...customization
    }

    loadTags()
    handleSSRRouter()

    let ver, lng

    const search = window.location.hash.split("?").pop()
    if (search && search !== '#/' && URLSearchParams) {
        const urlP = new URLSearchParams(search)
        ver = urlP.get("version")
        lng = urlP.get("lng")
    }
    ver = ver || window.localStorage.getItem("version") || 'develop'
    lng = lng || window.localStorage.getItem('lng') || 'zh-CN'

    const vTag = document.getElementById('tag_version')
    vTag.textContent = ver === 'develop' ? 'Latest' : ver
    const lTag = document.getElementById('tag_language')
    lTag.textContent = lngs[lng]
    if (window.location.hostname === 'localhost') {
        docsify.basePath = `./${lng}`
    } else {
        docsify.basePath = baseUrlGen(ver, lng)
    }
    setTimeout(() => {
        formatURLtoSSRRouter(lng, ver)
    }, 1000)
    window.$docsify = docsify
}

var main = function () {
    initDocsify()
}

main()
