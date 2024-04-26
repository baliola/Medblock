import DetailPatient, { DetailType } from '@/scenes/Detail/Detail.scene';
import { GetServerSideProps } from 'next';

export const getServerSideProps: GetServerSideProps<DetailType> = async (
  context,
) => {
  const query = context.query;
  const parsed = JSON.parse(JSON.stringify(query));
  console.log('session id server side props', parsed);

  return {
    props: {
      name: parsed.name,
      sessionId: parsed.id,
    },
  };
};

const DetailPage = (props: DetailType) => {
  console.log('server side props:', props);
  return <DetailPatient {...props} />;
};
DetailPage.patientLayout = true;
export default DetailPage;
