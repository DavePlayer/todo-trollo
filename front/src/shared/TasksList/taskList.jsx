import { useSelector } from "react-redux"

export const TasksList = ({ groupid }) => {
    const tasks = useSelector((state) => state.tasks);
    return (
        <div className="task-list" >
            {tasks.loading && <p>Loading...</p>}
            {tasks.data.filter(task => task.groupid === groupid).length === 0 && <p>No tasks yet!</p>}
            {!tasks.loading && tasks.data.filter(task => task.groupid === groupid).map(task => (
                task.isCrossed && 
                    <div className="task crossed" key={ task.id }>
                        <h3>{ task.title }</h3>
                        <p>{ task.text }</p>
                        <p>Created by: { task.owner }</p>
                    </div> ||
                !task.isCrossed && 
                    <div className="task" key={ task.id }>
                        <h3>{ task.title }</h3>
                        <p>{ task.text }</p>
                        <p>Created by: { task.owner }</p>
                    </div>
            ))}
        </div>
    )
}