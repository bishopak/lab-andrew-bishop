docker run --name labOne --detach -p8051:8500 smartqe/qe-shoppingcart-v1
docker run --name labTwo --detach -p8053:8500 smartqe/qe-student-v1
docker run --name labThree --detach -p8054:8500 smartqe/qe-vehicle-v1

docker ps -a