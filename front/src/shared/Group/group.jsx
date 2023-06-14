
export const Group = ({ title, owner }) => {
    return (
        <div className="group" >
            <h2> {title} </h2>
            <p>TASKS</p>
            <p>Created by: {owner}</p>
        </div>
    )
}