<template>
  <div id="search-div">
    <input id="search" />
    <button class="material-icons" id="search-button">search</button>
  </div>
</template>

<style lang="scss">
$gray: #b7b7b7;

#search-div {
  position: relative;
  display: flex;
  flex-direction: row;
  left: 16px;
  top: 16px;
  gap: 8px;
  color: $gray;
  #search {
    /* Frame 3 */
    display: flex;
    flex-direction: row;
    padding: 4px;
    padding-left: 8px;
    //   justify-content: center;
    //   align-content: center;
    box-sizing: border-box;
    position: relative;
    width: 250px;
    height: 36px;

    border: 1px solid $gray;
    border-radius: 5px;
  }
  #search-button {
    background-color: white;
    border: 0px;
    border-radius: 5px;
    &:hover {
      box-shadow: 0px 4px 4px rgba(0, 0, 0, 0.25);
      animation: shake 1s;
    }
  }
}
</style>

<script setup lang="ts"></script>
