# Python example demonstrating various patterns for code search

import json
import logging
from typing import Dict, List, Optional
from dataclasses import dataclass
from enum import Enum

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class TaskStatus(Enum):
    """Enumeration for task status"""
    PENDING = "pending"
    IN_PROGRESS = "in_progress"
    COMPLETED = "completed"
    CANCELLED = "cancelled"

@dataclass
class Task:
    """A simple task data class"""
    id: int
    title: str
    description: str
    status: TaskStatus
    priority: int = 1

class TaskManager:
    """A task management system"""
    
    def __init__(self):
        self.tasks: Dict[int, Task] = {}
        self.next_id = 1
    
    def create_task(self, title: str, description: str, priority: int = 1) -> Task:
        """Create a new task"""
        task = Task(
            id=self.next_id,
            title=title,
            description=description,
            status=TaskStatus.PENDING,
            priority=priority
        )
        self.tasks[self.next_id] = task
        self.next_id += 1
        logger.info(f"Created task: {task.title}")
        return task
    
    def update_task_status(self, task_id: int, status: TaskStatus) -> bool:
        """Update task status"""
        if task_id in self.tasks:
            self.tasks[task_id].status = status
            logger.info(f"Updated task {task_id} status to {status.value}")
            return True
        return False
    
    def get_task(self, task_id: int) -> Optional[Task]:
        """Get a task by ID"""
        return self.tasks.get(task_id)
    
    def list_tasks(self, status: Optional[TaskStatus] = None) -> List[Task]:
        """List all tasks, optionally filtered by status"""
        if status:
            return [task for task in self.tasks.values() if task.status == status]
        return list(self.tasks.values())
    
    def delete_task(self, task_id: int) -> bool:
        """Delete a task"""
        if task_id in self.tasks:
            del self.tasks[task_id]
            logger.info(f"Deleted task {task_id}")
            return True
        return False
    
    def export_tasks(self, filename: str) -> None:
        """Export tasks to JSON file"""
        tasks_data = []
        for task in self.tasks.values():
            tasks_data.append({
                'id': task.id,
                'title': task.title,
                'description': task.description,
                'status': task.status.value,
                'priority': task.priority
            })
        
        with open(filename, 'w') as f:
            json.dump(tasks_data, f, indent=2)
        
        logger.info(f"Exported {len(tasks_data)} tasks to {filename}")

def main():
    """Main function demonstrating task manager usage"""
    manager = TaskManager()
    
    # Create some tasks
    task1 = manager.create_task("Learn Rust", "Study Rust programming language", 3)
    task2 = manager.create_task("Write tests", "Add unit tests to the project", 2)
    task3 = manager.create_task("Code review", "Review pull requests", 1)
    
    # Update task status
    manager.update_task_status(task1.id, TaskStatus.IN_PROGRESS)
    manager.update_task_status(task2.id, TaskStatus.COMPLETED)
    
    # List tasks
    print("All tasks:")
    for task in manager.list_tasks():
        print(f"  {task.id}: {task.title} - {task.status.value}")
    
    # Export tasks
    manager.export_tasks("tasks.json")

if __name__ == "__main__":
    main()
