<script setup lang="ts">
import { warn, debug, info, trace, error } from '@winterfox/service/log'
import Button from 'primevue/button'

function info_button() {
  info('这是一条 info 信息')
}
function debug_button() {
  debug('这是一条 debug 信息')
}
function warn_button() {
  warn('这是一条 warn 信息')
}
function trace_button() {
  trace('这是一条 trace 信息')
}
function error_button() {
  error('这是一条 error 信息')
}
</script>

<template>
  <div class="winterfox-titlebar"></div>
  <div class="winterfox-main"></div>

  <Button label="Submit" />

  <button class="info" @click="info_button">info</button>
  <button class="debug" @click="debug_button">debug</button>
  <button class="warn" @click="warn_button">warn</button>
  <button class="trace" @click="trace_button">trace</button>
  <button class="error" @click="error_button">error</button>
</template>

<style lang="scss" scoped>
button {
  width: 160px;
  height: 90px;
  margin: 5px;
  padding: 10px;
  font-size: larger;
}

.info {
  background-color: rgb(241, 233, 233);
}
.debug {
  background-color: rgb(58, 133, 233);
}
.warn {
  background-color: rgb(223, 136, 66);
}
.trace {
  background-color: rgb(58, 236, 88);
}
.error {
  background-color: rgb(255, 0, 0);
}
</style>
