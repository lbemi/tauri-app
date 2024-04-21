<template>
  <form class="row" @submit.prevent="list_pods">
    <input id="namespace" v-model="state.namespace" placeholder="Enter a namespace..." />
    <button type="submit">Greet</button>
  </form>

  <el-table :data="state.pods">
    <el-table-column
      prop="metadata.name"
      label="Name"
      width="180"
    />
  	<el-table-column fixed="right" label="操作" width="150">
					<template #default="scope">
						<div>
							<el-button link type="primary" size="small" >详情</el-button><el-divider direction="vertical" />
							<el-button link type="primary" size="small" >编辑</el-button><el-divider direction="vertical" />
							<el-button link type="primary" size="small" @click="deletePod(scope.row)">删除</el-button>
						</div>
						<div>
							<el-button link type="primary" size="small" >终端</el-button><el-divider direction="vertical" />
							<el-button link type="primary" size="small" >日志</el-button><el-divider direction="vertical" />
							<el-button link type="primary" size="small" >文件</el-button>
						</div>
					</template>
				</el-table-column>
  </el-table>

  <p>{{ greetMsg }}</p>
</template>

<script setup lang="ts">
import {  reactive, ref,h } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { ElMessageBox, ElMessage } from 'element-plus';

const greetMsg = ref("");
const name = ref("");

const state = reactive({
  namespace: "default",
  pods: [],
})
async function list_pods() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
  await invoke("list_pods",{namespace: state.namespace}).then((res) => {
    state.pods = res
  }).catch((err) => {
    console.log(err)
  });
}

const deletePod = async (pod:any) => {
  
  ElMessageBox({
		title: '提示',
		message: h('p', null, [
			h('span', null, '此操作将删除 '),
			h('i', { style: 'color: teal' }, `${pod.metadata?.name}`),
			h('span', null, ' 容器. 是否继续? '),
		]),
		buttonSize: 'small',
		showCancelButton: true,
		confirmButtonText: '确定',
		cancelButtonText: '取消',
		type: 'warning',
		draggable: true,
	})
		.then( async () => {
			await invoke("delete_pod",{name: pod.metadata.name, namespace: pod.metadata.namespace})
				.then(() => {
					ElMessage({
						type: 'success',
						message: `${pod.metadata.name} 已删`,
					})
          list_pods();
          ;
				})
				.catch((e) => {
					ElMessage.error(e.message);
				});
		})
		.catch(() => {
			ElMessage.info('取消');
		});
  
}



</script>