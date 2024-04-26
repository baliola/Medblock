import MedicalRecord, {
  MedicalRecordType,
} from '@/scenes/MedicalRecord/MedicalRecord.scene';
import { GetServerSideProps } from 'next';

export const getServerSideProps: GetServerSideProps<MedicalRecordType> = async (
  context,
) => {
  const query = context.query;
  const parsed = JSON.parse(JSON.stringify(query));
  console.log('med record server side props', parsed);

  return {
    props: {
      providerId: parsed.providerId,
      sessionId: parsed.sessions,
      emrId: parsed.id,
    },
  };
};

const MedRecordPageEdit = (props: MedicalRecordType) => {
  console.log('med record  server side props:', props);
  return <MedicalRecord {...props} />;
};
MedRecordPageEdit.patientLayout = true;
export default MedRecordPageEdit;
