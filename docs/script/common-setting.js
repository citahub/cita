// this file is NOT for customization
// these are the default settings for all the Nervos Documents
// you can overwrite settings in this page by set them again in customization.js

// add language variable to session storage, if it's not existed
if (sessionStorage.getItem("language")) {
    var language = sessionStorage.getItem("language");
} else {
    sessionStorage.setItem("language", default_language);
    var language = default_language;
}

switch (language) {
    case "zh-CN":
        document.getElementById("tag_language").innerHTML = "中文";
        break;
    default:
        document.getElementById("tag_language").innerHTML = "English";
        break;
}

// add version variable to session storage, if it's not existed
if (sessionStorage.getItem("version")) {
    var version = sessionStorage.getItem("version");
} else {
    sessionStorage.setItem("version", "v0.18");
    var version = "v0.18";
}

document.getElementById("tag_version").innerHTML = version;

var common = {

    loadSidebar: true,
    autoHeader: true,
    subMaxLevel: 2,
    basePath: versionIsSupported ? `./${language}/${version}/` : `./${language}/`,


    // configuration for searching plugin
    search: {
        maxAge: 86400000, // expiration time in milliseconds, one day by default

        // depth of the maximum searching title levels
        depth: 6,
    },

    plugins: [
        function (hook, vm) {
            hook.afterEach(function (html, next) {
                if (versionIsSupported) {
                    var url = github_url + language + '/' + version + '/' + vm.route.file
                } else {
                    var url = github_url + language + '/' + vm.route.file
                }

                var editHtml = `<hr> If you find any mistakes on this page, feel free to <a target='_blank' href="${url}">edit this document on GitHub</a>`

                next(html + editHtml)
            })
        }
    ]


}