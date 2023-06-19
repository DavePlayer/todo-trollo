import { useSelector } from "react-redux";
import { useEffect, useState } from "react";
import { TasksList } from "../TasksList/taskList";
import { toast } from "react-toastify";
import "react-toastify/dist/ReactToastify.css";

export const Group = ({ groupid, name, creator }) => {
    const [creatorName, setCreatorName] = useState("");
    const user = useSelector((state) => state.user);
    useEffect(() => {
        console.log("getting there", groupid, name, creator);
        if (creator)
            fetch(`http://127.0.0.1:8080/user/${creator}`, {
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
                    console.log("dsaf");
                    const userData = await data.json();
                    setCreatorName(userData.name);
                })
                .catch((err) => {
                    console.error(err);
                    toast.error(err.message);
                });
    }, []);
    return (
        <div className="group">
            <h2> {name} </h2>
            <p>Created by: {creatorName}</p>
            <TasksList groupId={groupid} />
        </div>
    );
};
