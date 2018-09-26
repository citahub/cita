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
      item.innerHTML = `<a href="javascript:toVersion('${tagName}')">${tagName}</a>`
      fragment.appendChild(item)
    })
    tagList.appendChild(fragment)

  }
  if (fetch) {
    fetch('https://api.github.com/repos/cryptape/cita/tags').then(res => res.json()).then(tags => tags.slice(0, -5).filter(tag => !tag.name.endsWith('rc')).map(tag => tag.name)).then(tagNames => {
      appendTags(tagNames)
    })
  } else {
    appendTags(['v0.17', 'v0.18rc', 'v0.18'])
  }
}
