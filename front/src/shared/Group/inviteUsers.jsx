import { useDispatch, useSelector } from "react-redux";
import { useEffect, useState } from "react";
import { toast } from "react-toastify";

export const InviteUsers = ({ groupId, setVisibility, groupName }) => {
    const user = useSelector((root) => root.user);
    const [usersToInvite, setUsersToInvite] = useState([]);
    const dispatch = useDispatch();

    const handleCross = (e, taskId) => {
        if (task.crossed_by_id == user.id || task.crossed_by_id == null)
            dispatch(crossTask({ token: user.jwt, taskId }));
        else return toast.error("you can't de-cross task that you did not cross");
    };

    const forceInvite = (id) => {
        fetch(`http://127.0.0.1:8080/force-invite-users`, {
            method: "PATCH",
            headers: {
                Authorization: `Bearer ${user.jwt}`,
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                user_ids: [id],
                group_id: groupId,
            }),
        })
            .then(async (data) => {
                if (!data.ok) {
                    console.log(data);
                    throw new Error(`${data.status}: ${await data.text()}`);
                }
                toast.info(await data.text());
                setUsersToInvite((prev) => prev.filter((user) => user.id != id));
            })
            .catch((err) => {
                console.error(err);
                toast.error(err.message);
            });
    };

    useEffect(() => {
        fetch(`http://127.0.0.1:8080/users-for-invite/${groupId}`, {
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
                setUsersToInvite(userData);
            })
            .catch((err) => {
                console.error(err);
                toast.error(err.message);
            });
    }, [groupId]);
    return (
        <section>
            <h1>Invite Users to "{groupName}"</h1>
            <table className="users-grid">
                <thead>
                    <tr>
                        <th>Id</th>
                        <th>Image</th>
                        <th>UserName</th>
                        <th></th>
                    </tr>
                </thead>
                <tbody>
                    {usersToInvite.map((user) => (
                        <tr key={user.id}>
                            <td>{user.id}</td>
                            <td>
                                <figure>
                                    {user.img_url ? (
                                        <img src={user.img_url} alt="not found" />
                                    ) : (
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            width="24"
                                            height="24"
                                            viewBox="0 0 24 24"
                                        >
                                            <path d="M19 7.001c0 3.865-3.134 7-7 7s-7-3.135-7-7c0-3.867 3.134-7.001 7-7.001s7 3.134 7 7.001zm-1.598 7.18c-1.506 1.137-3.374 1.82-5.402 1.82-2.03 0-3.899-.685-5.407-1.822-4.072 1.793-6.593 7.376-6.593 9.821h24c0-2.423-2.6-8.006-6.598-9.819z" />
                                        </svg>
                                    )}
                                </figure>
                            </td>
                            <td>{user.name}</td>
                            <td>
                                <button onClick={() => forceInvite(user.id)}>Invite</button>
                            </td>
                        </tr>
                    ))}
                </tbody>
            </table>
            <div className="button-space">
                <button onClick={() => setVisibility(false)}>Return</button>
            </div>
        </section>
    );
};
