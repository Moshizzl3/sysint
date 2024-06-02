"""“add-data”

Revision ID: 32506dbefa5d
Revises: 51d76961fc8e
Create Date: 2024-06-02 16:18:06.378581

"""

from typing import Sequence, Union

from alembic import op
import sqlalchemy as sa
from sqlalchemy.sql import text


# revision identifiers, used by Alembic.
revision: str = "32506dbefa5d"
down_revision: Union[str, None] = "51d76961fc8e"
branch_labels: Union[str, Sequence[str], None] = None
depends_on: Union[str, Sequence[str], None] = None


def upgrade():
    # Detect the type of database, and use the appropriate SQL function for the current timestamp
    # bind = op.get_bind()
    # if bind.dialect.name == "sqlite":
    #     current_timestamp = "CURRENT_TIMESTAMP"
    # else:
    #     current_timestamp = "NOW()"

    # op.execute(
    #     text(
    #         f"""
    #     INSERT INTO students (name, age, note, create_at)
    #     VALUES 
    #     ('Alice Smith', 22, 'Excellent student, very proactive.', {current_timestamp}),
    #     ('Bob Johnson', 21, 'Great in team projects.', {current_timestamp}),
    #     ('Charlie Davis', 23, 'Highly creative and innovative.', {current_timestamp});
    # """
    #     )
    # )
    pass

def downgrade():
    # op.execute(
    #     """
    #     DELETE FROM students
    #     WHERE name IN ('Alice Smith', 'Bob Johnson', 'Charlie Davis');
    # """
    # )
    pass