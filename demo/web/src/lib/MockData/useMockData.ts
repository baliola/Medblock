'use-client';
const useMockData = (lgt: number, data: () => {}) => {
  const users = [];
  for (let i = 0; i < lgt; i++) {
    const datas = data();
    users.push(datas);
  }
  return users;
};

export default useMockData;
