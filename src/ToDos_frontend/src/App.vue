
<script>
import { ToDos_backend } from 'declarations/ToDos_backend/index';

export default {
  name: "App",
  data() {
    return {
      newTask: "",
      tasks: [], // Holds the list of tasks
    };
  },
  methods: {
    // Add a new task to the backend
    async addTask() {
      if (this.newTask.trim()) {
        console.log("Adding task:", this.newTask);
        try {
          await ToDos_backend.add_task(this.newTask); // Call add_task
          this.newTask = ""; // Clear input field
          await this.loadTasks(); // Reload the task list
        } catch (err) {
          console.error("Error adding task:", err);
        }
      }
    },

    // Load the list of tasks from the backend
    async loadTasks() {
      try {
        const taskList = await ToDos_backend.list_tasks(); // Fetch tasks
        this.tasks = taskList; // Update the tasks array
        console.log("Tasks loaded:", this.tasks);
      } catch (err) {
        console.error("Error fetching tasks:", err);
      }
    },

    // Mark a task as done
    async markTaskDone(id) {
  try {
    await ToDos_backend.mark_task_done(id); // Pass the correct task ID
    this.loadTasks(); // Reload the list after marking as done
  } catch (err) {
    console.error("Error marking task as done:", err);
  }
},

    // Delete a task
    async deleteTask(id) {
  try {
    await ToDos_backend.delete_task(id); // Pass the correct task ID
    this.loadTasks(); // Reload the list after deletion
  } catch (err) {
    console.error("Error deleting task:", err);
  }

  },
  },
  mounted() {
    this.loadTasks(); // Load tasks when the app is mounted
  },
  
};
</script>

<template>
  <div id="app">
    <h1>ICP Task Manager</h1>

    <!-- Add a New Task -->
    <div>
      <input
        type="text"
        v-model="newTask"
        placeholder="Enter a new task"
      />
      <button @click="addTask">Add Task</button>
    </div>

    <!-- Debugging Tasks -->
    <pre>{{ tasks }}</pre>

    <!-- Task List -->
    <h2>Task List</h2>
    <ul>
      <li
      v-for="(task, index) in tasks"
      :key="task.id"
      :class="{ done: task.done }"
        >
    <span>{{ task.description }}</span> <!-- Use task.description instead of task.name -->
    <button @click="markTaskDone(task.id)" v-if="!task.done">Mark Done</button>
    <button @click="deleteTask(task.id)">Delete</button>
  </li>
  </ul>
  </div>
</template>

<style>
.done {
  text-decoration: line-through;
}
</style>

