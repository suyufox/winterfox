import { invoke } from '@tauri-apps/api/core'

export async function debug(message: string) {
  await invoke('plugin:winterfox-service|debug', {
    message: {
      value: message
    }
  })
  console.debug(message)
}

export async function info(message: string) {
  await invoke('plugin:winterfox-service|info', {
    message: {
      value: message
    }
  })
  console.info(message)
}

export async function warn(message: string) {
  await invoke('plugin:winterfox-service|warn', {
    message: {
      value: message
    }
  })
  console.warn(message)
}

export async function trace(message: string) {
  await invoke('plugin:winterfox-service|trace', {
    message: {
      value: message
    }
  })
  console.trace(message)
}

export async function error(message: string) {
  await invoke('plugin:winterfox-service|error', {
    message: {
      value: message
    }
  })
  console.error(message)
}
