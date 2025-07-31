import './style.css'; // CSSファイルをインポート

function ListWithFadeEdge({ items }: {items: string[]}) {
    return (
        <div className="list-container">
            <ul className="list">
                {items.map((item, index) => (
                    <li key={index}>
                        <h1>{item}</h1>
                    </li>
                ))}
            </ul>
        </div>
    );
}

export default ListWithFadeEdge;