var log = console.log.bind(console);
var baseUrlGen = (ver, lng) => `https://raw.githubusercontent.com/cryptape/cita/${ver|| 'develop'}/docs/${lng || 'zh-CN'}/Latest`
