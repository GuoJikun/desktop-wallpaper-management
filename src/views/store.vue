<script setup lang="ts">
import { Setting } from '@element-plus/icons-vue'

const handleCommand = (command: string) => {
    console.log(command)
}
</script>

<template>
    <div class="store">
        <div class="store-header">
            <span>壁纸中心</span>
            <el-dropdown @command="handleCommand" class="store-setting">
                <el-link>
                    <el-icon size="20"><Setting /></el-icon>
                </el-link>
                <template #dropdown>
                    <el-dropdown-menu>
                        <el-dropdown-item command="source">设置壁纸源</el-dropdown-item>
                    </el-dropdown-menu>
                </template>
            </el-dropdown>
        </div>
        <div class="store-aside"></div>
        <div class="store-content"></div>
    </div>
</template>

<style lang="scss" scoped>
.store{
    min-height: 100%;
    display: grid;
    grid-template-rows: auto 1fr;      /* 第一行 header，高度自适应，第二行填满 */
    grid-template-columns: 240px 1fr;  /* aside 固定 240px，content 自适应 */
    grid-template-areas:
    "header header"
    "aside content";
    &-header{
        position: relative;
        display: flex;
        justify-content: center;
        align-items: center;
        grid-area: header;
        height: 4rem;
        padding: 0 2.4rem;
    }
    &-aside{
        grid-area: aside;
        background-color: beige;
    }
    &-content{
        grid-area: content;
        background-color: aliceblue;
    }

    &-setting {
        position: absolute;
        top: 50%;
        right: 2rem;
        transform: translateY(-50%);
    }
}

</style>
