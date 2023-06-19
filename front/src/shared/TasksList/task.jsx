import { useDispatch, useSelector } from "react-redux";
import { useEffect, useState } from "react";
import { TasksList } from "../TasksList/taskList";
import { toast } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";
import { crossTask } from "../../redux/reducers/tasks";

export const Task = ({ task }) => {
    const [crossUser, setCrossUser] = useState("");
    const user = useSelector((root) => root.user);
    const dispatch = useDispatch();

    const handleCross = (e, taskId) => {
        if (task.crossed_by_id == user.id || task.crossed_by_id == null)
            dispatch(crossTask({ token: user.jwt, taskId }));
        else return toast.error("you can't de-cross task that you did not cross");
    };

    useEffect(() => {
        if (task && (task.crossed_by_id || task.crossed_by_id == 0)) {
            fetch(`http://127.0.0.1:8080/user/${task.crossed_by_id}`, {
                method: "GET",
                headers: {
                    Authorization: `Bearer ${user.jwt}`,
                },
            })
                .then(async (data) => {
                    if (!data.ok) {
                        console.log(data);
                        throw new Error(`${data.status}: ${await data.text()}`);
                    }
                    const userData = await data.json();
                    setCrossUser(userData.name);
                })
                .catch((err) => {
                    console.error(err);
                    toast.error(err.message);
                });
        }
    }, [task.crossed_by_id]);
    return (
        <div className="task" onClick={(e) => handleCross(e, task.id)}>
            <h3 className={`${task.crossed_by_id != null ? "crossed" : ""}`}>{task.name}</h3>
            {task.crossed_by_id != null && <p>crossed by {crossUser}</p>}
        </div>
    );
};
