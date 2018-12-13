var log = console.log.bind(console);
var dir = (ver) => {
  if (['v0.17', 'v0.18'].indexOf(ver) > -1) {
    return 'Latest'
  }
  return ''
}
var baseUrlGen = (ver, lng) => `https://raw.githubusercontent.com/cryptape/cita/${ver|| 'develop'}/docs/${lng || 'zh-CN'}/${dir(ver)}`

const loadTags = () => {
  // let tag
  const tagList = document.querySelector("#tag-list")
  const fragment = document.createDocumentFragment()
  const appendTags = (tagNames) => {
    tagNames.forEach(tagName => {
      const item = document.createElement('li')
      item.innerHTML = `<a href="javascript:setVersion('${tagName}')">${tagName}</a>`
      fragment.appendChild(item)
    })
    tagList.appendChild(fragment)

  }
  if (fetch) {
    fetch('https://api.github.com/repos/cryptape/cita/tags').then(res => res.json()).then(tags => tags.slice(0, -5).filter(tag => !tag.name.endsWith('rc')).map(tag => tag.name)).then(tagNames => {
      window.tags = tagNames
      appendTags(tagNames)
    })
  } else {
    appendTags(['v0.17', 'v0.18rc', 'v0.18'])
  }
}


const handleSSRRouter = () => {
  const lngs = {
    en: 'en-US',
    zh: 'zh-CN'
  }
  const params = window.location.hash.replace('#/', '').split('/')
  if (Object.keys(lngs).indexOf(params[0]) > -1) {
    const fileRouter = !params[2] ? '' : params.slice(2).join('/')
    const newPath = `${window.location.hostname === 'localhost' ? '' : '/cita'}/#/` + fileRouter
    window.location.replace(newPath)
  }
}

const formatURLtoSSRRouter = (lng, ver) => {
  const lngs = {
    'en-US': 'en',
    'zh-CN': 'zh'
  }
  const path = window.location.hash.slice(2).split('?')[0] || ''
  const newPath = `${window.location.hostname === 'localhost' ? '' : '/cita'}/#/${lngs[lng]}/${ver}/${path}`
  window.history.replaceState("", "", newPath)
}
window.handleSSRRouter = handleSSRRouter
