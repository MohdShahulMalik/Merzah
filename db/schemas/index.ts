import { composeSchema } from 'smig';
import { users } from './users';
import { user_identifier } from './user_identifier';
import { sessions } from './sessions';
import { mosques } from './mosque';
import { handles } from './handles';

export default composeSchema( {
models: {
    users,
    user_identifier,
    sessions,
    mosques,
    handles,
}} );
