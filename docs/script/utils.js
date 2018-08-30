var log = console.log.bind(console);
var dir = (ver) => {
  if (ver === 'v0.17'||ver==='v0.18') {
    return 'Latest'
  }
  return ''
}
var baseUrlGen = (ver, lng) => `https://raw.githubusercontent.com/cryptape/cita/${ver|| 'develop'}/docs/${lng || 'zh-CN'}/${dir(ver)}`
