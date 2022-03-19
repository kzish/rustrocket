<template>
  <!-- Responsive navbar-->
        <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
            <div class="container">
                <a class="navbar-brand" href="#">Rust Todo App</a>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation"><span class="navbar-toggler-icon"></span></button>
                <div class="collapse navbar-collapse" id="navbarSupportedContent">
                    <ul class="navbar-nav ms-auto mb-2 mb-lg-0">
                        <!-- <li class="nav-item"><a class="nav-link active" aria-current="page" href="#">Home</a></li>
                        <li class="nav-item"><a class="nav-link" href="#">Link</a></li> -->
                        <!-- <li class="nav-item dropdown">
                            <a class="nav-link dropdown-toggle" id="navbarDropdown" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">Dropdown</a>
                            <ul class="dropdown-menu dropdown-menu-end" aria-labelledby="navbarDropdown">
                                <li><a class="dropdown-item" href="#">Action</a></li>
                                <li><a class="dropdown-item" href="#">Another action</a></li>
                                <li><hr class="dropdown-divider" /></li>
                                <li><a class="dropdown-item" href="#">Something else here</a></li>
                            </ul>
                        </li> -->
                    </ul>
                </div>
            </div>
        </nav>
        <!-- Page content-->
        <div class="container">
            <div class="text-center mt-5">
                <h1>A RustLang Todo Microservice</h1>
                <p class="lead">manage your todo list</p>
            </div>

            <div class="row">
              <div class="col-md-12">
                 <div class="form-group">
                  <label><h2>Add Description</h2></label>
                  <div class="input-group mb-3">
                    <input v-on:keyup.enter="createTask"  ref="task_description" type="text" class="form-control" placeholder="type description...">
                    <div class="input-group-append">
                      <button @click="createTask" class="btn btn-info">Add Task</button>
                    </div>
                  </div>
                </div>
              </div>
            </div>
            <hr/>
            <div class="row" v-if="tasks.length > 0">
              <div class="col-md-12">
                <h2>items</h2>
                  <table class="table table-hover">
                    <thead>
                      <tr>
                        <th scope="col">#</th>
                        <th scope="col">Description</th>
                        <th scope="col">Done</th>
                        <th scope="col">Action</th>
                      </tr>
                    </thead>
                    <tbody>
                      <tr v-for="(task, index) in tasks" :key="task.id">
                        
                        <th scope="row">
                          {{index+1}}
                        </th>

                        <!-- strike through text for completed task -->
                        <td :title="task.description" v-if="task.done==1">
                          <s> 
                           {{task.description}} 
                          </s>
                        </td>
                        <!-- no strike through text for completed task -->
                        <td :title="task.description" v-if="task.done==0">
                           {{task.description}} 
                        </td>

                        <td>
                          <input style="transform: scale(1.5);" @change="updateTask(task, $event.target)" type="checkbox" :checked="task.done==1">
                        </td>
                        <td>
                          <img @click="deleteTask(task)" src="assets/img/trash-can.svg" height="25" style="cursor:pointer;" title="remove" />
                        </td>
                      </tr>
                    </tbody>
                  </table>
              </div>
            </div>

            <div class="row" v-if="tasks.length == 0">
              <div class="col-md-12">
                <center>
                  <h2>No tasks</h2>
                </center>
              </div>
            </div>
        </div>


    <loading :active="isLoading" 
    :can-cancel="true" 
    :on-cancel="onCancel"
    :is-full-page="true" />

</template>

<script>

var querystring = require('querystring');
import {globals} from '@/assets/js/globals.js';
import axios from "axios";
import Loading from 'vue3-loading-overlay';
import 'vue3-loading-overlay/dist/vue3-loading-overlay.css';


export default {
  name: 'App',
  components: {
   Loading
  },
  data(){
    return {
        tasks: [],
        isLoading: false,
    }
  },
  mounted() {
    this.getTasks()
  },
  methods: {
    getTasks() {
        this.isLoading = true;
         axios.get(globals.api_end_point + "/getTodos")
          .then(response => {
            console.log(response);
              this.tasks = response.data;
          })
          .catch(error => 
          { 
            console.log(error);
            alert('Error occurred, fetching tasks');
          })
          .finally(() => {
            this.isLoading = false;
          })
    },
    createTask () { 

      if(this.$refs.task_description.value == "") {
        alert('Add a descrtiption');
        return;
      }

      const new_task = querystring.stringify({
            description: this.$refs.task_description.value, 
            done: 0 
      });

      this.isLoading = true;

      axios.post(globals.api_end_point + "/createTodo", new_task )
          .then(response => 
          {
            console.log('debug', response);
            this.$refs.task_description.value = "";
          }).catch(error => 
          { 
            console.log('error', error);
            alert('Error occurred, creating a task');
          })
          .finally(()=> {
            this.isLoading = false;
            this.getTasks()
          });
    },
    updateTask(task, checkbox) {
    //   if(this.$refs.task_description.value == "") {
    //   alert('Add a descrtiption');
    //   return;
    // }

      console.log(checkbox.checked);
      const new_task = querystring.stringify({
            id: task.id,
            // description: task.description,
            done: checkbox.checked ? 1 : 0
      });

      this.isLoading = true;

      axios.post(globals.api_end_point + "/updateTodo", new_task )
          .then(response => 
          {
            console.log('debug', response);
            this.$refs.task_description.value = "";
          }).catch(error => 
          { 
            console.log('error', error);
            alert('Error occurred, creating a task');
          })
          .finally(()=> {
            this.isLoading = false;
            this.getTasks()
          });
    },
    deleteTask(task) {
        this.isLoading = true;
        axios.post(globals.api_end_point + "/deleteTodo/" + task.id)
          .then(response => {
            console.log(response);
              this.tasks = response.data;
          })
          .catch(error => 
          { 
            console.log(error);
            alert('Error occurred, deleting task');
          })
          .finally(() => {
            this.isLoading = false;
            this.getTasks();
          })
    }
  }
}
</script>
