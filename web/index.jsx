function escapeHtml(unsafe) {
return unsafe
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#039;");
}

var LinkList = React.createClass({
    displayName: "LinkList",

    getInitialState() {
        return {
            submissions: [],
            comments: [],
        };
    },

    componentDidMount() {
        this.fetchSubmissions();
        this.fetchComments();
    },

    render() {
        return (
            <ul>{
                this.state.submissions.map((submission) => this.renderSubmission(submission))
            }</ul>
        );
    },

    renderSubmission(submission) {
        return (
            <li>
                <h2>
                    <a href={ submission.url }>{ submission.url }</a>
                </h2>
                {
                    this.renderReactionStrip(submission.id)
                }
                {
                    this.renderComments(submission.id)
                }
            </li>
        );
    },

    renderComments(thingId) {
        // There might not be any comments for this thing.
        let comments = this.state.comments[thingId] || [];
        // Discard all but the first comment of each type. We want to do clever things.
        comments = comments.filter((comment, index, self) => {
            const firstOfKind = self.find((c) => c.comment_plaintext === comment.comment_plaintext);
            firstOfKind.count = firstOfKind.count || 0;
            ++firstOfKind.count;
            return firstOfKind === comment;
        });

        if (comments.length === 0) {
            return false;
        }
        return (
            <div>
                <h3>Comments:</h3>
                <ul>
                    {
                        comments.map((comment) => this.renderComment(comment))
                    }
                </ul>
            </div>
        );
    },

    renderComment(comment) {
        const maybeCount = (comment.count > 1) && (
            <div className="commentCount">
                <span>
                    { comment.count }
                </span>
            </div>
        );
        return (
            <li>
                <span dangerouslySetInnerHTML={
                    {
                        __html: emojione.toImage(escapeHtml(comment.comment_plaintext)),
                    }
                }></span>
                {
                    maybeCount
                }
                {
                    this.renderReactionStrip(comment.id)
                }
                { this.renderComments(comment.id) }
            </li>
        );
    },

    renderReactionStrip(thingId) {
        const reactions = [
            ":thumbsup:",
            ":thumbsdown:",
            ":grinning:",
            ":grin:",
            ":joy:",
            ":heart_eyes:",
            ":stuck_out_tongue_winking_eye:",
            ":sunglasses:",
            ":flushed:",
            ":rage:",
            ":fearful:",
            ":sweat:",
            ":sob:",
            ":poop:",
            ":clap:",
        ];
        return (
            <div className="reaction-strip">
                {
                    reactions.map((reaction) => this.renderReactionButton(thingId, reaction))
                }
            </div>
        );
    },

    renderReactionButton(thingId, reaction) {
        return (
            <button
                onClick={ () => this.createComment(thingId, reaction) }
                dangerouslySetInnerHTML={
                    {
                        __html: emojione.toImage(escapeHtml(reaction)),
                    }
                }
            >
            </button>
        );
    },

    fetchSubmissions: function() {
        fetch("/api/submissions")
        .then(function(response) {
            return response.json();
        })
        .then((submissions) => {
            this.setState({
                submissions: submissions,
            });
        });
    },

    fetchComments() {
        fetch("/api/comments")
        .then((response) => response.json())
        .then((comments) => {
            // Hacks to preserve compatibility with existing API for quick path to finish line.
            const commentMap = {};
            comments.forEach((comment) => {
                commentMap[comment.parent_id] = commentMap[comment.parent_id] || [];
                commentMap[comment.parent_id].push(comment);
            });
            this.setState({
                comments: commentMap,
            });
        });
    },

    createComment(parentThingId, commentPlaintext) {
        fetch("/api/comments", {
            method: "POST",
            body: JSON.stringify({
                parent_id: parentThingId,
                comment_plaintext: commentPlaintext,
            })
        })
        .then((response) => {
            this.fetchComments();
        })
    },
});

var reactRoot = document.getElementById('react-root');
ReactDOM.render(<LinkList/>, reactRoot);
