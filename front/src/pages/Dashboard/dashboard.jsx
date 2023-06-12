import { useSelector } from "react-redux";
import { Navbar } from "../../shared/Navbar/navbar";

export const Dashboard = () => {
    const groups = useSelector((state) => state.groups);
    return(
        <div className="dashboard">
            <Navbar/>
            <div className="mainspace">
                {groups.loading && <p>Loading...</p>}
                {!groups.loading && groups.data.map(group => (
                    <div className="group" key={group.id}>
                        <h2> {group.title} </h2>
                        <p>TASKS</p>
                        <p>Created by: {group.owner}</p>
                    </div>
                ))}
            </div>
        </div>
    );
}
