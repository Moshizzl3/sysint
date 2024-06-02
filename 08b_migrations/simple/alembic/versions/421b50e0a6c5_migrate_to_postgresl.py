"""migrate_to_postgresl

Revision ID: 421b50e0a6c5
Revises: 32506dbefa5d
Create Date: 2024-06-02 16:23:48.092198

"""

from typing import Sequence, Union

from alembic import op
import sqlalchemy as sa


# revision identifiers, used by Alembic.
revision: str = "421b50e0a6c5"
down_revision: Union[str, None] = "32506dbefa5d"
branch_labels: Union[str, Sequence[str], None] = None
depends_on: Union[str, Sequence[str], None] = None

"""migrate_to_postgresl

Revision ID: 421b50e0a6c5
Revises: 32506dbefa5d
Create Date: 2024-06-02 16:23:48.092198

"""

from typing import Sequence, Union

from alembic import op
import sqlalchemy as sa


# revision identifiers, used by Alembic.
revision: str = "421b50e0a6c5"
down_revision: Union[str, None] = "32506dbefa5d"
branch_labels: Union[str, Sequence[str], None] = None
depends_on: Union[str, Sequence[str], None] = None


def upgrade():
    # Establish connections to both databases
    sqlite_engine = sa.create_engine("sqlite:///./sqlite.db")
    postgres_connection = (
        op.get_bind()
    )  # Assuming the Alembic context is bound to the PostgreSQL engine

    # Read data from SQLite
    sqlite_connection = sqlite_engine.connect()
    result = sqlite_connection.execute(sa.text("SELECT * FROM students"))

    # Insert data into PostgreSQL

    for row in result:
        print(row)
        postgres_connection.execute(
            sa.text(
                "INSERT INTO students (name, age, note, create_at) VALUES (:name, :age, :note, :create_at)"
            ),
            parameters={
                "name": row[1],
                "age": row[2],
                "note": row[3],
                "create_at": row[4],
            },
        )

    # Close connections


def downgrade():
    # Optionally, implement logic to move data back to SQLite if needed
    pass
