<template>
  <div class="content-main">
    <div class="activite-bar">
      <div
        class="activite-item-container"
        :class="{ 'activite-item-choiced': i.choosed }"
        v-for="i in items"
      >
        <span
          :class="{
            'material-icons-outlined': !i.choosed,
            'material-icons': i.choosed,
          }"
          class="activite-item"
          @click="click(i)"
          >{{ i.icon }}</span
        >
      </div>
    </div>
    <div class="content"><component :is="router" /></div>
  </div>
</template>

<script setup lang="ts">
import { Ref, computed, ref } from "vue";
import home from "./content/home.vue";

let routes: { [name: string]: any } = {
  Home: home,
};

let router = computed(() => {
  return routes[choosing];
});

let choosing = "Home";
let items: Ref<{
  [name: string]: { icon: string; title: string; choosed: boolean };
}> = ref({
  Home: { icon: "home", title: "Home", choosed: true },
  Proxys: { icon: "adjust", title: "Proxys", choosed: false },
  Settings: { icon: "settings", title: "Settings", choosed: false },
});

function click(i: { icon: string; title: string; choosed: boolean }) {
  if (choosing == i.title) {
    return;
  }
  i.choosed = true;
  items.value[choosing].choosed = false;
  choosing = i.title;
}
</script>

<style scoped>
.content-main {
  display: flex;
  flex-direction: row;
  position: absolute;
  top: 64px;
  width: 100%;

  bottom: 0;
}

.activite-bar {
  float: left;
  display: flex;
  flex-direction: column;
  width: 48px;
  padding-bottom: 0px;
  /* height: 100%; */
  background-color: #f6f8fc;
  align-items: flex-start;
  padding: 8px;
  gap: 10px;
}

.activite-item {
  display: block;
  left: 16px;
  user-select: none;
  -moz-user-select: none;
  -webkit-user-select: none;
  font-size: 32px;
  color: #444746;
  margin: auto;
}

.activite-item-container {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 48px;
  height: 48px;
}

.activite-item-choiced {
  /* color: white; */
  box-shadow: 0px 4px 4px rgba(0, 0, 0, 0.25);
  border-radius: 5px;
}

.content {
  border-radius: 20px;
  float: left;
  background-color: white;
  width: 100%;
  height: 100%;
}
</style>
