import { useSelector } from "react-redux";
import { useEffect, useState } from "react";
import { TasksList } from "../TasksList/taskList";
import { toast } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";

export const Task = ({ task }) => {
    const [crossUser, setCrossUser] = useState("");
    const user = useSelector((root) => root.user);

    useEffect(() => {
        if (task && (task.crossed_by_id || task.crossed_by_id == 0)) {
            console.log("fetching user somwhow when it shouldn't ", task);
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
    }, []);
    return (
        <div className="task">
            <h3 className={`${task.crossed_by_id != null ? "crossed" : ""}`}>{task.name}</h3>
            {task.crossed_by_id != null && <p>crossed by {crossUser}</p>}
        </div>
    );
};
