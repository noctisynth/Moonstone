<script setup lang="ts">
import Button from "primevue/button";
import Checkbox from "primevue/checkbox";
import InputText from "primevue/inputtext";
import Password from "primevue/password";
import IconField from 'primevue/iconfield';
import InputIcon from 'primevue/inputicon';
import Card from 'primevue/card';
import Stepper from 'primevue/stepper';
import StepperPanel from 'primevue/stepperpanel';
import Inplace from 'primevue/inplace';

import { invoke } from "@tauri-apps/api/core";
import { useField, useForm } from 'vee-validate';
import { onMounted, ref } from "vue";
import { useRouter } from 'vue-router';
import { useThemeStore } from "../stores/theme";
import { usePrimeVue } from "primevue/config";
import Steps from "primevue/steps";
import ProgressSpinner from "primevue/progressspinner";
import { useDebugStore } from "../stores/debug";
import { useLoginStore } from "../stores/login";

const router = useRouter();
const debugstore = useDebugStore()
const loginstore = useLoginStore()

const { handleSubmit: handleCheck, resetForm: _resetCheckForm } = useForm();
const { value: node, errorMessage: invalidNode } = useField('node', validateNode);
const { value: licenseAccept, errorMessage: invalidLicenseAccept } = useField('username', validateLicenseAccept);

if (loginstore.node) {
  node.value = loginstore.node
}

const { handleSubmit, resetForm: _resetForm } = useForm();
const { value: identity, errorMessage: invalidUsername } = useField('username', validateUsername);
const { value: password, errorMessage: invalidPassword } = useField('password', validatePassword);
const { value: accept, errorMessage: invalidAccept } = useField('accept', validateAccept);

const info = ref<string>();
const error = ref<string>();

function validateLicenseAccept(value: string) {
  if (!value) {
    return "请先同意月长石使用协议！";
  }

  return true;
}


function validateNode(value: string) {
  if (!value) {
    return "请输入服务端地址！";
  }

  return true;
}

function validateUsername(value: string) {
  if (!value) {
    return "请输入用户名！";
  }

  return true;
}

function validatePassword(value: string) {
  console.log()
  if (!value) {
    return "请输入密钥！";
  }

  return true;
}

function validateAccept(value: boolean) {
  if (!value) {
    return "请阅读并同意浊莲文明协定！";
  }

  return true;
}

const login = handleSubmit(async () => {
  if (debugstore.debug) {
    router.push("/dashboard")
  }
  info.value = "登录中...";

  let callback: { status: boolean; session_key: string; error: string } = JSON.parse(
    await invoke("login_handler", { server: node.value, identity: identity.value, password: password.value })
  );
  if (callback["status"]) {
    localStorage.setItem("server", node.value);
    localStorage.setItem("isLoggedIn", "true");
    localStorage.setItem("session_key", callback["session_key"]);
    info.value = "登录成功";
    router.push('/');
  } else {
    info.value = "";
    error.value = callback["error"];
  }
})

const themestore = useThemeStore()
const PrimeVue = usePrimeVue()
const active = ref(0)
const checkStep = ref(0)
const checkStatus = ref<string | null>()
const inCheckProgress = ref<boolean>(true)
const inProgress = ref<boolean>(false)
const allChecksOk = ref<boolean>(false)
const checkFailedMessage = ref<string | null>()
const allChecks = ref([
  {
    label: '网络连接'
  },
  {
    label: '系统环境'
  },
  {
    label: '网络环境'
  },
])

const checkNode = handleCheck(() => {
  inProgress.value = true;
  loginstore.checkNode(node.value, true)
  active.value = 2
  inProgress.value = false
})

async function checkEnvironment() {
  if (debugstore.debug) {
    inCheckProgress.value = false
    allChecksOk.value = true
    checkFailedMessage.value = "您当前处于开发者模式！"
    return
  }
  checkStep.value = 0
  checkStatus.value = null
  inCheckProgress.value = true
  checkFailedMessage.value = null
  allChecksOk.value = false
  checkStatus.value = "检查网络连接中..."
  const internet = await invoke("check_internet")
  if (internet) {
    checkStatus.value = "检查系统环境中..."
    checkStep.value = 1;
    const system = await invoke("check_system")
    if (system) {
      checkStep.value = 2;
      checkStatus.value = "检查网络环境中..."
      const security = await invoke("check_security")
      inCheckProgress.value = false
      if (security) {
        checkStatus.value = "环境检查完毕."
        allChecksOk.value = true;
      } else {
        inCheckProgress.value = false
        checkStatus.value = null;
        checkFailedMessage.value = "网络环境存在风险，请确保你在安全的网络环境登入，否则你可能遭到网络审查或网络攻击！"
      }
    } else {
      inCheckProgress.value = false
      checkFailedMessage.value = "系统环境存在异常，请确保你在安全的设备登入！"
      checkStatus.value = null;
    }
  } else {
    inCheckProgress.value = false
    checkFailedMessage.value = "网络连接失败，请检查你的网络连接。"
    checkStatus.value = null;
  }
}
onMounted(async () => {
  await checkEnvironment()
})
</script>

<template>
  <main class="w-full h-full">
    <div class="flex flex-column justify-content-center align-items-center h-full w-full">
      <div class="max-w-full max-h-full">
        <div class="flex w-full justify-content-start align-items-start pl-2">
          <Button @click="themestore.changeTheme(PrimeVue)" v-if="!themestore.dark" icon="pi pi-sun" plain
            text></Button>
          <Button @click="themestore.changeTheme(PrimeVue)" v-else icon="pi pi-moon" plain text></Button>
        </div>

        <Stepper linear v-model:activeStep="active" class="h-full max-w-full pb-4" style="width: 618px">
          <StepperPanel header="检查">
            <template #content="{ nextCallback }">
              <Card class="p-3">
                <template #title>
                  <h2>环境检查</h2>
                </template>
                <template #subtitle>
                  {{ checkStatus }}
                </template>
                <template #content>
                  <div class="flex flex-column gap-4">
                    <div class="flex justify-content-center align-items-center">
                      <ProgressSpinner v-if="inCheckProgress"></ProgressSpinner>
                      <div v-else v-if="checkFailedMessage !== null"
                        class="flex flex-column align-items-start justify-content-center">
                        <p class="p-error">{{ checkFailedMessage }}</p>
                        <Inplace>
                          <template #display>
                            <Button class="p-0" label="更多选项" size="small" plain text></Button>
                          </template>
                          <template #content>
                            <Button @click="checkEnvironment" icon="pi pi-sync" label="重新检查" size="small" plain
                              text></Button>
                            <Inplace class="p-1">
                              <template #display>
                                <span class="pi pi-exclamation-triangle text-gray-400 pr-1"></span>
                                <span class="text-gray-400 text-sm">无视风险</span>
                              </template>
                              <template #content>
                                <div class="flex flex-row gap-3">
                                  <Checkbox v-model="allChecksOk" :binary="true"></Checkbox> <span
                                    class="p-error text-sm">我已知悉可能存在的风险，愿意承担可能的一切后果。</span>
                                </div>
                              </template>
                            </Inplace>
                          </template>
                        </Inplace>
                      </div>
                    </div>
                    <div class="flex justify-content-center">
                      <Steps class="w-full" :model="allChecks" v-model:activeStep="checkStep" />
                    </div>
                  </div>
                </template>
                <template #footer>
                  <div v-if="allChecksOk" class="flex pt-4 justify-content-end">
                    <Button label="连接" icon="pi pi-arrow-right" iconPos="right" @click="nextCallback"></Button>
                  </div>
                </template>
              </Card>
            </template>
          </StepperPanel>
          <StepperPanel header="节点">
            <template #content>
              <Card class="p-3">
                <template #title>
                  <div class="flex justify-content-center">
                    <h2>节点</h2>
                  </div>
                </template>
                <template #content>
                  <div class="flex flex-column justify-content-center gap-2">
                    <form @submit="checkNode" class="flex flex-column gap-3">
                      <IconField>
                        <InputIcon class="pi pi-globe"></InputIcon>
                        <InputText id="node-input" v-model="node" class="w-full" placeholder="节点地址" :disabled="inProgress"/>
                      </IconField>
                      <small v-if="invalidNode" class="p-error" id="node-error">{{ invalidNode
                        }}</small>
                      <div class="field-checkbox">
                        <Checkbox v-model="licenseAccept" :binary="true" inputId="license-accept-input"
                          aria-describedby="license-accept-error" :disabled="inProgress"/>
                        <label for="license-accept-input">我已阅读并同意月长石使用协议</label>
                      </div>
                      <small v-if="invalidLicenseAccept" class="p-error" id="license-accept-error">{{
            invalidLicenseAccept
          }}</small>
                      <small v-if="info" class="p-info" id="text-info">{{ info }}</small>
                      <small v-if="error" class="p-error" id="text-error">{{ error }}</small>
                      <div class="flex pt-4 justify-content-end">
                        <Button type="submit" label="登入"
                          :icon="(inProgress ? 'pi pi-spin pi-spinner' : 'pi pi-arrow-right')" iconPos="right"></Button>
                      </div>
                    </form>
                  </div>
                </template>
              </Card>
            </template>
          </StepperPanel>
          <StepperPanel header="登入">
            <template #content="{ prevCallback }">
              <Card class="p-3">
                <template #title>
                  <div class="flex justify-content-center">
                    <h2>登入</h2>
                  </div>
                </template>
                <template #content>
                  <div class="flex flex-column justify-content-center gap-2">
                    <form @submit="login" class="flex flex-column gap-3">
                      <IconField>
                        <InputIcon class="pi pi-globe"></InputIcon>
                        <InputText v-model="loginstore.node" class="w-full" placeholder="节点地址" disabled />
                      </IconField>
                      <IconField>
                        <InputIcon class="pi pi-user"></InputIcon>
                        <InputText id="username-input" v-model="identity" class="w-full" placeholder="用户名或邮箱" />
                      </IconField>
                      <small v-if="invalidUsername" class="p-error" id="username-error">{{ invalidUsername
                        }}</small>
                      <Password v-model="password" type="text" inputId="password-input" :feedback="false" class="w-full"
                        aria-describedby="password-error" toggleMask placeholder="密码" />
                      <small v-if="invalidPassword" class="p-error" id="password-error">{{ invalidPassword
                        }}</small>
                      <div class="field-checkbox">
                        <Checkbox v-model="accept" :binary="true" name="accept" inputId="accept-input"
                          :class="{ 'p-invalid': invalidAccept }" aria-describedby="accept-error" />
                        <label for="accept-input">我已阅读并同意浊莲文明协定</label>
                      </div>
                      <small v-if="invalidAccept" class="p-error" id="accept-error">{{ invalidAccept
                        }}</small>
                      <small v-if="info" class="p-info" id="text-info">{{ info }}</small>
                      <small v-if="error" class="p-error" id="text-error">{{ error }}</small>
                      <div class="flex pt-4 justify-content-between">
                        <Button label="返回" icon="pi pi-arrow-left" severity="secondary" @click="prevCallback"></Button>
                        <Button label="认证" icon="pi pi-arrow-right" type="submit" iconPos="right"></Button>
                      </div>
                    </form>
                  </div>
                </template>
              </Card>
            </template>
          </StepperPanel>
        </Stepper>
      </div>
    </div>
  </main>
</template>

<style scoped>
:deep(.p-password-input) {
  width: 100%;
}

:deep(.p-stepper .p-stepper-nav) {
  overflow-x: hidden;
}

:deep(.p-steps .p-steps-item .p-menuitem-link .p-steps-title) {
  font-size: 0.9rem;
}
</style>
