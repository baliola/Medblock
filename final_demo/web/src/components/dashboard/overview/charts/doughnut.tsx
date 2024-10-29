import { useOverviewStore } from "@/store/overview-store";
import { Flex, Text } from "@chakra-ui/react";
import { Chart as ChartJS, ArcElement, Tooltip, Legend } from "chart.js";
import { Doughnut } from "react-chartjs-2";

ChartJS.register(ArcElement, Tooltip, Legend);

export default function DoughtnutChart() {
  const patient_age = useOverviewStore(state => state.overview?.patient_age);
  if (!patient_age) return null;

  const data = {
    labels: ["0-18", "19-30", "31-45", "46-60", "60+"],
    datasets: [
      {
        label: "Patient Age",
        data: [
          patient_age["0-18"],
          patient_age["19-30"],
          patient_age["31-45"],
          patient_age["46-60"],
          patient_age["60+"]
        ],
        backgroundColor: [
          "#FF6384",
          "#36A2EB",
          "#FFCE56",
          "#FFCE56",
          "#FFCE56"
        ],
        hoverBackgroundColor: [
          "#FF6384",
          "#36A2EB",
          "#FFCE56",
          "#FFCE56",
          "#FFCE56"
        ]
      }
    ]
  };

  const options: any = {
    responsive: true,
    plugins: {
      legend: {
        position: 'top',
      },
    },
  };

  return (
    <Flex
      direction={"column"}
      align={'center'}
      justify={'center'}
      bg={"primary.100"}
      p={12} gap={5}
      rounded={"3xl"}
      h={"70dvh"}
      w={'full'}
    >
      <Text fontSize={'xl'} fontWeight={'bold'}>
        Age of Patients
      </Text>
      <Doughnut data={data} width={500} height={500} options={options} />
    </Flex>
  )
}