import { useOverviewStore } from "@/store/overview-store";
import { Flex, Text } from "@chakra-ui/react";

const Statistic = ({
  title, value
}: {
  title: string, value: string
}) => {
  return (
    <Flex
      direction={"column"}
      gap={1}
      color={"neutral.700"}
      bg={"primary.100"}
      p={8}
      rounded={"xl"}
      w={'lg'}
    >
      <Text>{title}</Text>
      <Text fontSize={"4xl"} fontWeight={"bold"}>
        {value}
      </Text>
    </Flex>
  )
}

const GenderRatioBar = ({
  male, female
}: {
  male: number,
  female: number
}) => {
  const total = male + female;
  const malePercentage = (male / total) * 100;
  const femalePercentage = (female / total) * 100;

  return (
    <Flex
      w="full"
      h="16"
      bg="primary.100"
      rounded="2xl"
      overflow="hidden"
      position="relative"
    >
      <Flex
        w={`${malePercentage}%`}
        h="full"
        bg="primary.400"
        align="center"
        justify="center"
        position="relative"
        zIndex="1"
      >
        <Text fontSize="lg" fontWeight="bold" color="neutral.700">
          {`${malePercentage.toFixed(2)}% Male`}
        </Text>
      </Flex>
      <Flex
        w={`${femalePercentage}%`}
        h="full"
        bg="accent.400"
        align="center"
        justify="center"
        position="relative"
        zIndex="1"
      >
        <Text fontSize="lg" fontWeight="bold" color="neutral.700">
          {`${femalePercentage.toFixed(2)}% Female`}
        </Text>
      </Flex>
    </Flex>
  )
}

export default function OverviewStats() {
  const overview = useOverviewStore(state => state.overview);
  if (!overview) return null;

  const parsingValue = (value: number) => {
    return Intl.NumberFormat('en', {
      notation: 'compact'
    }).format(value);
  }

  return (
    <Flex direction={"column"} gap={5}>
      <Flex gap={5} align={'center'}>
        <Statistic
          title="Patients"
          value={parsingValue(overview.patients)}
        />
        <Statistic
          title="E-Medical Records"
          value={parsingValue(overview.emr)}
        />
        <Statistic
          title="Doctors"
          value={parsingValue(overview.doctors)}
        />
      </Flex>

      <Flex
        direction={'column'}
        w={"5xl"}
        bg={"primary.100"}
        rounded={"3xl"}
        p={5} gap={5}
      >
        <Text fontSize={"2xl"} fontWeight={"bold"}>
          Patient Gender Ratio
        </Text>
        <GenderRatioBar
          male={overview.patient_gender.male}
          female={overview.patient_gender.female}
        />
      </Flex>
    </Flex>
  )
}