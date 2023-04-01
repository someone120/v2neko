<template>
  <div class="appbar">
    <div class="material-icons" id="appbar-more" @click="show = !show">more_horiz</div>
    <p id="appbar-text" >V2Neko</p>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
const show = ref(false);
</script>

<style scoped>
@font-face {
  font-family: NotoSans;
  src: url("../assets/fonts/NotoSansSC-Medium.otf");
}

.appbar {
  position: relative;
  height: 64px;
  background: #F6F8FC;
}

#appbar-more {
  position: absolute;
  width: 36px;
  height: 36px;
  position: absolute;
  width: 36px;
  height: 36px;
  left: 16px;
  top: 14px;
  font-size: 34px;
  color: #444746;
  user-select: none;
}

#appbar-text {
  /* V2Neko */
  position: absolute;
  width: 255px;
  height: 36px;
  left: 72px;
  top: -14px;

  font-family: "NotoSans";
  font-style: normal;
  font-weight: 400;
  font-size: 24px;
  line-height: 39px;

  color:#444746;
}

</style>
