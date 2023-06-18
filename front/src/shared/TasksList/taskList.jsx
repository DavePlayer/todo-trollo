import { useSelector } from "react-redux";
import { useEffect, useState } from "react";
import { Task } from "./task";

export const TasksList = ({ groupId }) => {
    const tasks = useSelector((root) => root.tasks);
    useEffect(() => {
        // get async reducer tasks
    }, []);
    return (
        <div className="task-list">
            {tasks.loading && <p>Loading...</p>}
            {tasks.data.filter((task) => task.group_id === groupId).length === 0 && (
                <p>No tasks yet!</p>
            )}
            {!tasks.loading &&
                tasks.data
                    .filter((task) => task.group_id === groupId)
                    .map((task) => <Task task={task} key={task.id} />)}
        </div>
    );
};
