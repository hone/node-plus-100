module.exports = (payload) => {
  console.log(`original value, ${payload} from middleware!`)
  payload += 100
  console.log(`new value, ${payload} from middleware!`)

  return (payload)
}
