import { TasksList } from "../TasksList/taskList"

export const Group = ({ groupid, title, owner }) => {
    return (
        <div className="group" >
            <h2> {title} </h2>
            <TasksList groupid={groupid}/>
            <p>Created by: {owner}</p>
        </div>
    )
}